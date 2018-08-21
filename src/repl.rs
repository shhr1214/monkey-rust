use std::io::{BufRead, BufReader, Read, Result, Write};

use lexer::Lexer;
use token;

const PROMPT: &str = ">> ";

pub fn start<T: Read, U: Write>(input: T, output: U) {
    let mut bf = BufReader::new(input);
    for line in bf.by_ref().lines() {
        let mut lexer = Lexer::new(line.unwrap());
        loop {
            let token = lexer.next_token();
            if token.token_type() == token::EOF {
                break;
            }
            println!("{:?}", token);
        }
    }
}
