use std::io::{BufReader, Read, Result, Write};

use lexer::Lexer;
use token;

const PROMPT: &str = ">> ";

pub fn start<T: Read, U: Write>(input: T, output: U) -> Result<usize> {
    let mut bf = BufReader::new(input);
    loop {
        println!("{}", PROMPT);
        let mut buf = vec![];
        let n = bf.read(&mut buf)?;
        if n == 0 {
            return Ok(0);
        }

        let mut lexer = Lexer::new(String::from_utf8(buf).unwrap_or("".to_string()));
        loop {
            let token = lexer.next_token();
            if token.token_type() != token::EOF {
                break;
            }
            println!("{:?}", token);
        }
    }
}
