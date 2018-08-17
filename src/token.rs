pub const ILLEGAL: &'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";

// 識別子 + リテラル
pub const IDENT: &'static str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &'static str = "INT"; // 1234567890

// 演算子
pub const ASSIGN: &'static str = "=";
pub const PLUS: &'static str = "+";

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

type TokenType = &'static str;

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

    pub fn token_type(self) -> TokenType {
        self.token_type
    }
}
