use token;
use token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub enum Statement {
    LetStatement(Identifier, Expression),
    ExpressionStatemen(Expr),
}

struct Expression {}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn statements(self) -> Vec<Statement> {
        self.statements
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            match self.statements[0] {
                Statement::LetStatement(_, _) => token::LET.to_string(),
            }
        } else {
            "".to_string()
        }
    }
}

pub struct Identifier(String);

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}
