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
            _ => panic!("Unrecognized Token: {}", self.ch as char),
        };
        self.read_char();
        token
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

    #[test]
    fn basic_tokens() {
        let input = "=+(){},;";
        let lexer = Lexer::new(input);
        let tokens = lexer.lex();
        assert_eq!(
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
            tokens
        );
    }
}
