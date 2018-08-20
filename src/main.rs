mod lexer;
mod repl;
mod token;

use std::io;

fn main() {
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout());
}
