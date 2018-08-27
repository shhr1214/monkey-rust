use ast::{Node, Program, Statement};
use lexer::Lexer;
use token;
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
        let statements = program.statements();
        assert_eq!(statements.len(), 3, "num of program statements must be 3");

        let tests = vec!["x", "y", "foobar"];
        for (i, t) in tests.iter().enumerate() {
            let stmt = &statements[i];
            assert!(test_let_statement(stmt, t.to_string()))
        }
    }

    fn test_let_statement(stmt: &Statement, name: String) -> bool {
        let tl = stmt.token_literal();
        if tl != token::LET.to_string() {
            return false;
        }

        match stmt {
            Statement::LetStatement(i, e) => {
                if i.0 != name {
                    return false;
                }
                if i.token_literal() != name {
                    return false;
                }
                true
            }
            _ => false,
        }
    }
}
