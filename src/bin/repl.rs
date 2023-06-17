use std::io;

use monkey::lexer::lexer::{Token, Lexer};

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    println!("Starting repl, press ^Z or ^D to exit");

    let mut line = String::new();

    while let Ok(_) = stdin.read_line(&mut line) {
        let mut lex = Lexer::new(line.clone().into());
        line.clear();
        let mut tok = lex.next_token();
        loop {
            println!("{}", tok);
            if tok == Token::Eof {
                break;
            }
            tok = lex.next_token();
        }
        println!("");
    }

    println!("Exiting repl");

    return Ok(());
}

