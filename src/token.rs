#![macro_use]
extern crate lazy_static;

use std::collections::HashMap;

const ILLEGAL: &'static str = "ILLEGAL";
const EOF: &'static str = "EOF";

// 識別子 + リテラル
const IDENT: &'static str = "IDENT"; // add, foobar, x, y, ...
const INT: &'static str = "INT"; // 1234567890

// 演算子
const ASSIGN: &'static str = "=";
const PLUS: &'static str = "+";

// デリミタ
const COMMA: &'static str = ",";
const SEMICOLON: &'static str = ";";

const LPAREN: &'static str = "(";
const RPAREN: &'static str = ")";
const LBRACE: &'static str = "{";
const RBRACE: &'static str = "}";

// キーワード
const FUNCTION: &'static str = "FUNCTION";
const LET: &'static str = "LET";

type TokenType = String;

struct Token {
    token_type: TokenType,
    literal: String,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", "FUNCTION");
        m.insert("let", "LET");
    };
}