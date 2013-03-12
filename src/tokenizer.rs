pub struct TokenData {
    string: ~str,
    row: uint,
    col: uint
}

pub enum Token {
    pub Number(TokenData),
    pub Newline(TokenData)
}

struct CharReader {
    source: ~str,
    next: uint,
    current_char: option::Option<char>,
    row: uint,
    col: uint
}

impl CharReader {

    static fn new(input_string: ~str) -> CharReader {
        CharReader {
            source: input_string,
            next: 0,
            current_char: option::None,
            row: 0,
            col: 0
        }
    }

    fn read_char(&mut self) {
        if self.next < self.source.len() {
            let str::CharRange {ch, next} = str::char_range_at(self.source, self.next);
            self.next = next;
            self.row += 1;
            self.current_char = option::Some(ch);
        } else {
            self.current_char = option::None;
        }
    }

    fn wind_past_whitespace(&mut self) {
        loop {
            match self.current_char {
                option::Some(' ') => {
                    self.read_char();
                },
                _ => {
                    break;
                }
            }
        }
    }
    
    fn wind_past_comments(&mut self) {
        match self.current_char {
            option::Some('⍝') => {
                loop {
                    match self.current_char {
                        option::Some(char) => {
                            if char == '\n' || char == '\r' {
                                break;
                            }
                            self.read_char()
                        },
                        option::None => {
                            break;
                        }
                    }
                }
            },
            _ => () 
        }
    }
}

struct Tokenizer {
    char_reader: @mut CharReader,
}

impl Tokenizer {
    static fn new(input_string: ~str) -> Tokenizer {
        let char_reader = CharReader::new(input_string);
        Tokenizer {
            char_reader: @mut char_reader
        }
    }

    pub fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        self.char_reader.read_char();
        self.char_reader.wind_past_whitespace();
        self.char_reader.wind_past_comments();
        match self.char_reader.current_char {
            option::Some(first_char) => {
                if NewlineTokenizer::is_valid_newline_start(first_char) {
                    let mut tokenizer = NewlineTokenizer::new(self.char_reader);
                    return tokenizer.read_next_token()
                }
                if NumberTokenizer::is_valid_number_start(first_char) {
                    let mut tokenizer = NumberTokenizer::new(self.char_reader);
                    return tokenizer.read_next_token()
                }
                result::Err(~"No valid token found")
            },
            option::None => {
                result::Err(~"End of file")
            }
        }
    }

}

struct NumberTokenizer {
    char_reader: @mut CharReader,
    period_encountered: bool,
    first_character: bool
}

impl NumberTokenizer {

    static fn is_valid_number_start(char: char) -> bool {
        //Needs to be either upper dash, period, or 0-9
        (char >= '0' && char <= '9') || char == '.' || char == '¯'
    }

    static fn new(char_reader: @mut CharReader) -> NumberTokenizer {
        NumberTokenizer {
            char_reader: char_reader,
            period_encountered: false,
            first_character: true
        }
    }

    fn is_period(&self) -> bool {
        match self.char_reader.current_char {
            option::Some('.') => true,
            _ => false
        }
    }

    fn is_number(&self) -> bool {
        match self.char_reader.current_char {
            option::Some(maybe_number) => {
                maybe_number >= '0' && maybe_number <= '9'
            },
            _ => false
        }
    }

    fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        let mut token: ~[char] = ~[];
        loop {
            if self.first_character {
                self.first_character = false;
                if self.is_period() {
                    self.period_encountered = true;
                }
                token.push(option::unwrap(self.char_reader.current_char));
            } else if self.is_period() {
                if self.period_encountered {
                    return result::Err(~"Invalid number");
                } else {
                    self.period_encountered = true;
                    token.push(option::unwrap(self.char_reader.current_char));
                }
            } else if self.is_number() {
                token.push(option::unwrap(self.char_reader.current_char));
            } else {
                if (token[token.len() - 1] == '.') {
                    return result::Err(~"Invalid number");
                }
                return result::Ok(Number(TokenData {
                    string: str::from_chars(token),
                    row: 0,
                    col: 0
                }));
            }
            self.char_reader.read_char();
        }
    }
}

struct NewlineTokenizer {
    char_reader: @mut CharReader,
    initial_next: uint, //TODO: Make this cache thing an object
    initial_char: option::Option<char>,
    initial_row: uint,
    initial_col: uint
}

impl NewlineTokenizer {

    static fn is_valid_newline_start(char: char) -> bool {
        char == '\n' || char == '\r'
    }

    static fn new(char_reader: @mut CharReader) -> NewlineTokenizer {
        NewlineTokenizer {
            char_reader: char_reader,
            initial_next: char_reader.next,
            initial_char: char_reader.current_char,
            initial_row: char_reader.row,
            initial_col: char_reader.col
        }
    }

    fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        match self.initial_char {
            option::Some('\r') => {
                self.char_reader.read_char();
                match self.char_reader.current_char {
                    option::Some('\n') => {
                        return result::Ok(Newline(TokenData {
                            string: ~"\r\n",
                            row: 0,
                            col: 0
                        }));
                    },
                    _ => {
                        self.char_reader.next = self.initial_next;
                        self.char_reader.current_char = self.initial_char;
                        self.char_reader.row = self.initial_row;
                        self.char_reader.col = self.initial_col;
                        return result::Ok(Newline(TokenData {
                            string: ~"\r",
                            row: 0,
                            col: 0
                        }));
                    }
                }
            },
            _ => {
                return result::Ok(Newline(TokenData {
                    string: ~"\n",
                    row: 0,
                    col: 0
                }));
            }
        }
    }
}
