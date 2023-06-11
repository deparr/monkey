use std::io;
use ::lexer::lexer::{Lexer, Token};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    println!("Startin repl...");

    let mut line = String::new();

    while let Ok(_) = io::stdin().read_line(&mut line) {
        let mut lex = Lexer::new();
        let mut tok = Token::Eof;
        loop {
            println!("tok: {:?}", tok);
            if tok == Token::Eof {
                break;
            }
        }
    }

    return Ok(());
}

