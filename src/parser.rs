use ast::Program;
use lexer::Lexer;
use token::Token;

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer: lexer,
            cur_token: cur_token,
            peek_token: peek_token,
        }
    }

    fn next_token(mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&self) -> Program {
        Program::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(
            program.statements().len(),
            3,
            "num of program statements must be 3"
        );

        let tests = vec!["x", "y", "foobar"];

        let statements = program.statements();
        for (i, t) in tests.iter().enumerate() {
            let stmt = &statements[i];
        }
    }
}
