use token;
use token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    // 入力における現在の位置
    position: u64,
    // これから読み込む位置
    read_position: u64,
    // 現在検査中の文字
    ch: u8,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let ch = input.chars().nth(0).unwrap_or(char::from(0)) as u8;
        Lexer {
            input: input,
            position: 0,
            read_position: 1, // 初期化時点で一文字読んでいる
            ch: ch,
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u64 {
            self.ch = 0;
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position as usize)
                .unwrap_or(char::from(0)) as u8;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = char::from(self.ch);
        let token = match ch {
            '=' => Token::new(token::ASSIGN, ch.to_string()),
            '+' => Token::new(token::PLUS, ch.to_string()),
            '-' => Token::new(token::MINUS, ch.to_string()),
            '!' => Token::new(token::BANG, ch.to_string()),
            '/' => Token::new(token::SLASH, ch.to_string()),
            '*' => Token::new(token::ASTERISK, ch.to_string()),
            '<' => Token::new(token::LT, ch.to_string()),
            '>' => Token::new(token::GT, ch.to_string()),
            ';' => Token::new(token::SEMICOLON, ch.to_string()),
            '(' => Token::new(token::LPAREN, ch.to_string()),
            ')' => Token::new(token::RPAREN, ch.to_string()),
            ',' => Token::new(token::COMMA, ch.to_string()),
            '{' => Token::new(token::LBRACE, ch.to_string()),
            '}' => Token::new(token::RBRACE, ch.to_string()),
            _ => {
                if is_null(ch) {
                    return Token::new(token::EOF, "".to_string());
                }
                if is_letter(ch) {
                    let literal = self.read_identifier();
                    return Token::new(token::lookup_ident(literal.as_str()), literal.to_string());
                } else if is_digit(ch) {
                    let literal = self.read_number();
                    return Token::new(token::INT, literal.to_string());
                } else {
                    return Token::new(token::ILLEGAL, ch.to_string());
                }
            }
        };

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        loop {
            match char::from(self.ch) {
                ' ' | '\t' | '\n' | '\r' => self.read_char(),
                _ => return,
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(char::from(self.ch)) {
            self.read_char();
        }
        self.input
            .chars()
            .skip(pos as usize)
            .take((self.position - pos) as usize)
            .collect()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while is_digit(char::from(self.ch)) {
            self.read_char();
        }
        self.input
            .chars()
            .skip(pos as usize)
            .take((self.position - pos) as usize)
            .collect()
    }
}

fn is_null(ch: char) -> bool {
    ch == char::from(0)
}

fn is_letter(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z')
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

#[cfg(test)]
mod tests {
    use super::*;
    use token;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;
"#;

        let tests = vec![
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, "="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "ten"),
            (token::ASSIGN, "="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "add"),
            (token::ASSIGN, "="),
            (token::FUNCTION, "fn"),
            (token::LPAREN, "("),
            (token::IDENT, "x"),
            (token::COMMA, ","),
            (token::IDENT, "y"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::IDENT, "x"),
            (token::PLUS, "+"),
            (token::IDENT, "y"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "result"),
            (token::ASSIGN, "="),
            (token::IDENT, "add"),
            (token::LPAREN, "("),
            (token::IDENT, "five"),
            (token::COMMA, ","),
            (token::IDENT, "ten"),
            (token::RPAREN, ")"),
            (token::SEMICOLON, ";"),
            (token::BANG, "!"),
            (token::MINUS, "-"),
            (token::SLASH, "/"),
            (token::ASTERISK, "*"),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::INT, "5"),
            (token::LT, "<"),
            (token::INT, "10"),
            (token::GT, ">"),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::EOF, ""),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for (i, test) in tests.iter().enumerate() {
            let token = lexer.next_token();
            let t = token.token_type().to_string();
            let l = token.literal();
            assert_eq!(
                t, test.0,
                "token type test: No. {} {:?} {:?}",
                i, lexer, token
            );
            assert_eq!(l, test.1, "literal test: No. {} {:?} {:?}", i, lexer, token);
        }
    }
}
