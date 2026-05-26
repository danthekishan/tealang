use std::env;

mod lexer;
mod repl;
mod token;

fn main() {
    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| String::from("there"));

    println!("Hello {}! This is tealang programming language", username);
    println!("Feel free to type in commands\n");

    repl::start();
}
