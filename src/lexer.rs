use token;
use token::Token;

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
        Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: 0,
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
        let ch = char::from(self.ch);
        let _eof = char::from(0);
        let token = match ch {
            '=' => Token::new(token::ASSIGN, ch.to_string()),
            ';' => Token::new(token::SEMICOLON, ch.to_string()),
            '(' => Token::new(token::LPAREN, ch.to_string()),
            ')' => Token::new(token::RPAREN, ch.to_string()),
            ',' => Token::new(token::COMMA, ch.to_string()),
            '+' => Token::new(token::PLUS, ch.to_string()),
            '{' => Token::new(token::LBRACE, ch.to_string()),
            '}' => Token::new(token::RBRACE, ch.to_string()),
            _eof => Token::new(token::EOF, "".to_string()),
            _ => Token::new(token::EOF, "".to_string()),
        };

        self.read_char();
        token
    }
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
            (token::EOF, ""),
        ];

        let mut l = Lexer::new(input.to_string());
        for test in tests.iter() {
            let token = l.next_token();
            assert_eq!(token.token_type(), test.0)
        }
    }
}
