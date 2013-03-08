pub struct TokenData {
    string: ~str,
    row: uint,
    col: uint
}

pub enum Token {
    Number(TokenData)
}

struct Tokenizer {
    source: ~str,
    next: uint,
    current_token: ~[char],
    current_char: option::Option<char>,
    row: uint,
    col: uint
}

impl Tokenizer {
    static fn new(input_string: ~str) -> Tokenizer {
        Tokenizer {
            source: input_string,
            next: 0,
            current_token: ~[],
            current_char: option::None,
            row: 0,
            col: 0
        }
    }

    pub fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        self.read_char();
        result::Err(~"Undefined")
    }

    fn read_char(&mut self) {
        if self.next < self.source.len() {
            let str::CharRange {ch, next} = str::char_range_at(self.source, self.next);
            self.next = next;
            self.current_char = option::Some(ch);
        } else {
            self.current_char = option::None;
        }
    }

}

