use std::io::{self, Write, stdin, stdout};

use crate::{lexer::Lexer, token::TokenType};

const PROMPT: &str = ">>";

fn read_input() -> io::Result<String> {
    print!("{PROMPT}");
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}

fn print_tokens(line: &str) {
    let mut lexer = Lexer::new(line);

    while let token = lexer.next_token()
        && token.token_type != TokenType::Eof
    {
        println!("out: {:?}", token);
    }
}

pub fn start() {
    loop {
        match read_input() {
            Ok(line) => print_tokens(&line),
            Err(e) => eprintln!("Encountered an error: {e}"),
        }
    }
}
