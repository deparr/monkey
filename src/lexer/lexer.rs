
use std::str;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof(String),

    Ident(String),
    Number(String),

    Let,
    Function,
    Return,
    If,
    Else,
    True,
    False,

    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lsquirly,
    Rsquirly,
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

    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();

        let mut is_multi = false;
        let token = match self.ch {
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,

            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            },
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b'{' => Token::Lsquirly,
            b'}' => Token::Rsquirly,
            b'A'..=b'Z' | b'a'..=b'z' => {
                is_multi = true;
                self.read_ident()
            },
            b'0'..=b'9' => {
                is_multi = true;
                self.read_number()
            },
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
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        let ident = str::from_utf8(&self.input[pos..self.position]).unwrap();
        return match ident {
            "let" => Token::Let,
            "fn" => Token::Function,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident.into()),
        };
    }

    fn read_number(&mut self) -> Token {
        //let mut digits = vec![];
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            //digits.push(self.ch & 0x0f);
            self.read_char();
        }

        let number = str::from_utf8(&self.input[pos..self.position]).unwrap();

        return Token::Number(number.into());
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }

        return self.input[self.read_position];
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
        let input = "(){}=+-/*!!===<>,;";

        let expected = vec![
            Token::Lparen,
            Token::Rparen,
            Token::Lsquirly,
            Token::Rsquirly,
            Token::Assign,
            Token::Plus,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Bang,
            Token::NotEqual,
            Token::Equal,
            Token::LessThan,
            Token::GreaterThan,
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

let result = add(five, ten);

if (5 < 10) {
    return true;
} else {
    return false;
}"#;

        println!{"input\n{}", input};

        let expected = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Number("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Number("10".into()),
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
            Token::Comma,
            Token::Ident("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Number("5".into()),
            Token::LessThan,
            Token::Number("10".into()),
            Token::Rparen,
            Token::Lsquirly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rsquirly,
            Token::Else,
            Token::Lsquirly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rsquirly,
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

