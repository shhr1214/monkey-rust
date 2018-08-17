struct Lexer {
    input: String,
    // 入力における現在の位置
    position: int64,
    // これから読み込む位置
    read_position: int64,
    // 現在検査中の文字
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }
}
