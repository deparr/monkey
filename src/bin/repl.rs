use std::io;

use monkey::lexer::lexer::{Token, Lexer};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    println!("Startin repl...");

    let mut line = String::new();

    while let Ok(_) = stdin.read_line(&mut line) {
        let mut lex = Lexer::new(line.clone().into());
        let mut tok = lex.next_token();
        loop {
            println!("tok: {:?}", tok);
            if tok == Token::Eof("".into()) {
                break;
            }
            tok = lex.next_token();
        }
    }

    return Ok(());
}

