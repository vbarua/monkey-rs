#[derive(Debug, PartialEq)]
enum TokenType {
    Illegal,
    EOF,

    // Identifiers + Literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

#[derive(Debug, PartialEq)]
struct Token(TokenType, Vec<char>);

// ASCII Only
struct Lexer {
    input: Vec<u8>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_number(c: char) -> bool {
    c.is_digit(10)
}

fn is_keyword(s: &str) -> Option<Token> {
    match s {
        "fn" => Some(Token(TokenType::Function, "fn".chars().collect())),
        "let" => Some(Token(TokenType::Let, "let".chars().collect())),
        _ => None,
    }
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.as_bytes().to_vec(),
            position: 0,
            read_position: 0,
            ch: b'\0',
        };
        lexer.read_char(); // Initialize Lexer
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.consume_whitespace();

        let token: Token = match self.ch {
            b'=' => Token(TokenType::Assign, vec![self.ch as char]),
            b';' => Token(TokenType::Semicolon, vec![self.ch as char]),
            b'(' => Token(TokenType::LParen, vec![self.ch as char]),
            b')' => Token(TokenType::RParen, vec![self.ch as char]),
            b',' => Token(TokenType::Comma, vec![self.ch as char]),
            b'+' => Token(TokenType::Plus, vec![self.ch as char]),
            b'{' => Token(TokenType::LBrace, vec![self.ch as char]),
            b'}' => Token(TokenType::RBrace, vec![self.ch as char]),
            b'\0' => Token(TokenType::EOF, vec![self.ch as char]),
            ch => {
                if is_letter(ch as char) {
                    let value = self.read_identifier();
                    if let Some(token) = is_keyword(&value) {
                        return token;
                    } else {
                        return Token(TokenType::Ident, value.chars().collect());
                    }
                } else if is_number(ch as char) {
                    return Token(TokenType::Int, self.read_number());
                } else {
                    Token(TokenType::Illegal, vec![ch as char])
                }
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch as char) {
            self.read_char();
        }
        let identifier_bytes = &self.input[position..self.position];
        identifier_bytes.iter().map(|byte| *byte as char).collect()
    }

    fn read_number(&mut self) -> Vec<char> {
        let position = self.position;
        while is_number(self.ch as char) {
            self.read_char();
        }
        let number_bytes = &self.input[position..self.position];
        number_bytes.iter().map(|byte| *byte as char).collect()
    }

    fn consume_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }

    fn lex(mut self) -> Vec<Token> {
        let mut token = self.next_token();
        let mut tokens: Vec<Token> = Vec::new();
        while token.0 != TokenType::EOF {
            tokens.push(token);
            token = self.next_token();
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_tokens(left: Vec<Token>, right: Vec<Token>) {
        let token_iter = left.iter().zip(right.iter()).enumerate();
        for (index, (left_token, right_token)) in token_iter {
            if left_token != right_token {
                assert_eq!(left[..index + 1], right[..index + 1]);
            }
        }

        assert_eq!(left.len(), right.len());
    }

    #[test]
    fn basic_tokens() {
        let input = "=+(){},;";
        let lexer = Lexer::new(input);
        let tokens = lexer.lex();
        compare_tokens(
            vec![
                Token(TokenType::Assign, vec!['=']),
                Token(TokenType::Plus, vec!['+']),
                Token(TokenType::LParen, vec!['(']),
                Token(TokenType::RParen, vec![')']),
                Token(TokenType::LBrace, vec!['{']),
                Token(TokenType::RBrace, vec!['}']),
                Token(TokenType::Comma, vec![',']),
                Token(TokenType::Semicolon, vec![';']),
            ],
            tokens,
        );
    }

    #[test]
    fn bindings_and_functions() {
        let input = "
        let five = 5;
            let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten)
    ";
        let lexer = Lexer::new(input);
        let tokens = lexer.lex();
        compare_tokens(
            vec![
                Token(TokenType::Let, "let".chars().collect()),
                Token(TokenType::Ident, "five".chars().collect()),
                Token(TokenType::Assign, vec!['=']),
                Token(TokenType::Int, "5".chars().collect()),
                Token(TokenType::Semicolon, vec![';']),
                Token(TokenType::Let, "let".chars().collect()),
                Token(TokenType::Ident, "ten".chars().collect()),
                Token(TokenType::Assign, vec!['=']),
                Token(TokenType::Int, "10".chars().collect()),
                Token(TokenType::Semicolon, vec![';']),
                Token(TokenType::Let, "let".chars().collect()),
                Token(TokenType::Ident, "add".chars().collect()),
                Token(TokenType::Assign, vec!['=']),
                Token(TokenType::Function, "fn".chars().collect()),
                Token(TokenType::LParen, vec!['(']),
                Token(TokenType::Ident, "x".chars().collect()),
                Token(TokenType::Comma, vec![',']),
                Token(TokenType::Ident, "y".chars().collect()),
                Token(TokenType::RParen, vec![')']),
                Token(TokenType::LBrace, vec!['{']),
                Token(TokenType::Ident, "x".chars().collect()),
                Token(TokenType::Plus, vec!['+']),
                Token(TokenType::Ident, "y".chars().collect()),
                Token(TokenType::Semicolon, vec![';']),
                Token(TokenType::RBrace, vec!['}']),
                Token(TokenType::Semicolon, vec![';']),
                Token(TokenType::Let, "let".chars().collect()),
                Token(TokenType::Ident, "result".chars().collect()),
                Token(TokenType::Assign, vec!['=']),
                Token(TokenType::Ident, "add".chars().collect()),
                Token(TokenType::LParen, vec!['(']),
                Token(TokenType::Ident, "five".chars().collect()),
                Token(TokenType::Comma, vec![',']),
                Token(TokenType::Ident, "ten".chars().collect()),
                Token(TokenType::RParen, vec![')']),
            ],
            tokens,
        );
    }
}
