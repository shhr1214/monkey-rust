pub const ILLEGAL: &'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";

// 識別子 + リテラル
pub const IDENT: &'static str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &'static str = "INT"; // 1234567890

// 演算子
pub const ASSIGN: &'static str = "=";
pub const PLUS: &'static str = "+";
pub const MINUS: &'static str = "-";
pub const BANG: &'static str = "!";
pub const ASTERISK: &'static str = "*";
pub const SLASH: &'static str = "/";

pub const LT: &'static str = "<";
pub const GT: &'static str = ">";

pub const EQ: &'static str = "==";
pub const NOT_EQ: &'static str = "!=";

// デリミタ
pub const COMMA: &'static str = ",";
pub const SEMICOLON: &'static str = ";";

pub const LPAREN: &'static str = "(";
pub const RPAREN: &'static str = ")";
pub const LBRACE: &'static str = "{";
pub const RBRACE: &'static str = "}";

// キーワード
pub const FUNCTION: &'static str = "FUNCTION";
pub const LET: &'static str = "LET";
pub const TRUE: &'static str = "TRUE";
pub const FALSE: &'static str = "FALSE";
pub const IF: &'static str = "IF";
pub const ELSE: &'static str = "ELSE";
pub const RETURN: &'static str = "RETURN";

type TokenType = &'static str;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type: token_type,
            literal: literal,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn literal(&self) -> String {
        self.literal.clone()
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => FUNCTION,
        "let" => LET,
        "true" => TRUE,
        "false" => FALSE,
        "if" => IF,
        "else" => ELSE,
        "return" => RETURN,
        _ => IDENT,
    }
}
