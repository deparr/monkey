use anyhow::Result;
use std::str;

#[derive(Debug, PartialEq)]
enum Token {
    Illegal,
    Eof(String),

    Ident(String),
    Int(isize),

    Function,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lsquirly,
    Rsquirly,
    Let,
}

#[derive(Debug)]
pub struct Lexer {
    read_position: usize,
    position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        let mut lexer = Lexer {
            read_position: 0,
            position: 0,
            ch: 0,
            input,
        };

        lexer.read_char();
        return lexer;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut is_multi = false;
        let token = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::Lsquirly,
            b'}' => Token::Rsquirly,
            b'A'..=b'Z' | b'a'..=b'z' => { is_multi = true; self.read_ident() },
            b'0'..=b'9' => { is_multi = true; self.read_number() },
            b'\0' => Token::Eof("".into()),
            _ => Token::Illegal,
        };

        if !is_multi {
            self.read_char();
        }

        return token;
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        };

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_ident(&mut self) -> Token {
        let cur_position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        let ident = str::from_utf8(&self.input[cur_position..self.position]).unwrap();
        return match ident {
            "fn" => Token::Function,
            "let" => Token::Let,
            _ => Token::Ident(ident.into()),

        }
    }

    fn read_number(&mut self) {
        let mut cur_position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }


    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

}

#[cfg(test)]
mod test {

    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn test_next_token() -> Result<()> {
        let input = "=+(){},;";

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lsquirly,
            Token::Rsquirly,
            Token::Comma,
            Token::Semicolon,
            Token::Eof("".into()),
        ];

        let mut lexer = Lexer::new(input.into());

        for token in expected {
            let next = lexer.next_token();
            println!("got {:?}, expected {:?}", next, token);
            assert_eq!(token, next);
        }

        return Ok(());
    }

    #[test]
    fn test_next_token_complete() -> Result<()> {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five + ten);
"#;

        let expected = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::Rparen,
            Token::Lsquirly,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::Rsquirly,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::Lparen,
            Token::Ident("five".into()),
            Token::Plus,
            Token::Ident("ten".into()),
            Token::Semicolon,
            Token::Rsquirly,
            Token::Semicolon,
            Token::Eof("".into()),
        ];

        let mut lexer = Lexer::new(input.into());

        for token in expected {
            let next = lexer.next_token();
            println!("expected {:?}, got {:?}", token, next);
            assert_eq!(token, next);
        }

        return Ok(());
    }
}

