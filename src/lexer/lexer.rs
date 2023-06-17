
use std::{str, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(String),

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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Ident(id) => write!(f, "Ident(\"{}\")", id),
            Token::Int(x) => write!(f, "Int(\"{}\")", x),
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "EOF"),
            Token::Let => write!(f, "Let"),
            Token::Function => write!(f, "Function"),
            Token::Return => write!(f, "Return"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::Assign => write!(f, "Assign"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::Bang => write!(f, "Bang"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Lparen => write!(f, "Lparen"),
            Token::Rparen => write!(f, "Rparen"),
            Token::Lsquirly => write!(f, "Lsquirly"),
            Token::Rsquirly => write!(f, "Rsquirly"),
        };
    }
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
                self.read_int()
            },
            b'\0' => Token::Eof,
            _ => Token::Illegal,
        };

        if !is_multi {
            self.read_char();
        }

        return token;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        let mut token = self.next_token();
        while token != Token::Eof {
            tokens.push(token);
            token = self.next_token();
        }
        return tokens;
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

    fn read_int(&mut self) -> Token {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let number = str::from_utf8(&self.input[pos..self.position]).unwrap();

        return Token::Int(number.into());
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
            Token::Eof,
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
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
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
            Token::Int("5".into()),
            Token::LessThan,
            Token::Int("10".into()),
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
            Token::Eof,
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

