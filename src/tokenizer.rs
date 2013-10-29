use std::{str, char};

pub struct TokenData {
    string: ~str,
    row: uint,
    col: uint
}

pub enum Token {
    Number(TokenData),
    Newline(TokenData),
    String(TokenData),
    Primitive(TokenData),
    Variable(TokenData),
    EndOfFile()
}

struct Backtrack {
    initial_next: uint,
    initial_char: Option<char>,
    initial_row: uint,
    initial_col: uint
}

struct CharReader {
    source: ~str,
    next: uint,
    current_char: Option<char>,
    row: uint,
    col: uint
}

impl CharReader {

    pub fn new(input_string: ~str) -> CharReader {
        CharReader {
            source: input_string,
            next: 0,
            current_char: None,
            row: 0,
            col: 0
        }
    }

    fn read_and_stash_char(&mut self) {
        if self.next < self.source.len() {
            let str::CharRange {ch, next} = self.source.char_range_at(self.next);
            self.next = next;
            self.row += 1;
            self.current_char = Some(ch);
        } else {
            self.current_char = None;
        }
    }

    fn wind_past_whitespace(&mut self) {
        loop {
            match self.current_char {
                Some(' ') => {
                    self.read_and_stash_char();
                },
                _ => {
                    break;
                }
            }
        }
    }
    
    fn wind_past_comments(&mut self) {
        match self.current_char {
            Some('⍝') => {
                loop {
                    match self.current_char {
                        Some(char) => {
                            if char == '\n' || char == '\r' {
                                break;
                            }
                            self.read_and_stash_char()
                        },
                        None => {
                            break;
                        }
                    }
                }
            },
            _ => () 
        }
    }

    fn create_backtrack(&self) -> Backtrack {
        Backtrack {
            initial_next: self.next,
            initial_char: self.current_char,
            initial_row: self.row,
            initial_col: self.col
        }
    }

    fn backtrack(&mut self, backtrack: &Backtrack) {
        self.next = backtrack.initial_next;
        self.current_char = backtrack.initial_char;
        self.row = backtrack.initial_row;
        self.col = backtrack.initial_col;
    }
}

struct Tokenizer {
    char_reader: ~CharReader,
}

impl Tokenizer {
    pub fn new(input_string: ~str) -> Tokenizer {
        let mut char_reader = CharReader::new(input_string);
        char_reader.read_and_stash_char();
        Tokenizer {
            char_reader: ~char_reader
        }
    }

    pub fn read_next_token(&mut self) -> Result<~Token, ~str> {
        self.char_reader.wind_past_whitespace();
        self.char_reader.wind_past_comments();
        match self.char_reader.current_char {
            Some(first_char) => {
                //FIXME: Make Tokenizer a trait, turn this into two lines of code!
                if is_valid_newline_start(first_char) {
                    return newline_tokenizer(self.char_reader)
                }
                if is_dot(first_char) {
                    return dot_tokenizer(self.char_reader)
                }
                if is_valid_number_start(first_char) {
                    return number_tokenizer(self.char_reader)
                }
                if is_valid_string_start(first_char) {
                    return string_tokenizer(self.char_reader)
                }
                if is_valid_primitive_start(first_char) {
                    return primitive_tokenizer(self.char_reader)
                }
                if is_valid_variable_start(first_char) {
                    return variable_tokenizer(self.char_reader)
                }
                Err(format!("No valid token found starting with {}", first_char))
            },
            None => {
                Ok(~EndOfFile)
            }
        }
    }
}

fn is_valid_number_start(char: char) -> bool {
    //Needs to be either upper dash, period, or 0-9
    (char >= '0' && char <= '9') || char == '.' || char == '¯'
}

fn is_period(char_reader: &CharReader) -> bool {
    match char_reader.current_char {
        Some('.') => true,
        _ => false
    }
}

fn is_number(char_reader: &CharReader) -> bool {
    match char_reader.current_char {
        Some(maybe_number) => {
            maybe_number >= '0' && maybe_number <= '9'
        },
        _ => false
    }
}

fn is_complex(char_reader: &CharReader) -> bool {
    match char_reader.current_char {
        Some('J') => true,
        _ => false
    }
}

fn is_negative(char_reader: &CharReader) -> bool {
    match char_reader.current_char {
        Some('¯') => true,
        _ => false
    }
}

fn number_tokenizer(char_reader: &mut CharReader) -> Result<~Token, ~str> {
    let mut period_encountered = false;
    let mut complex_encountered = false;
    let mut first_character = true;
    let mut allowed_negative = false;
    let mut token: ~[char] = ~[];

    loop {
        if first_character {
            first_character = false;
            if is_period(char_reader) {
                period_encountered = true;
            }
            token.push(char_reader.current_char.unwrap());
        } else if is_negative(char_reader) {
            if allowed_negative {
                allowed_negative = false;
                token.push(char_reader.current_char.unwrap());
            } else {
                return Err(~"Invalid number");
            }
        } else if is_complex(char_reader) {
            if complex_encountered {
                return Err(~"Invalid number");
            } else {
                complex_encountered = true;
                period_encountered = false;
                allowed_negative = true;
                token.push(char_reader.current_char.unwrap());
            }
        } else if is_period(char_reader) {
            allowed_negative = false;
            if period_encountered {
                return Err(~"Invalid number");
            } else {
                period_encountered = true;
                token.push(char_reader.current_char.unwrap());
            }
        } else if is_number(char_reader) {
            allowed_negative = false;
            token.push(char_reader.current_char.unwrap());
        } else {
            if (token[token.len() - 1] == '.' ||
                token[token.len() - 1] == 'J' ||
                token[token.len() - 1] == '¯') {
                return Err(~"Invalid number");
            }
            return Ok(~Number(TokenData {
                string: str::from_chars(token),
                row: 0,
                col: 0
            }));
        }
        char_reader.read_and_stash_char();
    }
}

fn is_valid_newline_start(char: char) -> bool {
    char == '\n' || char == '\r'
}

fn newline_tokenizer(char_reader: &mut CharReader) -> Result<~Token, ~str> {
    match char_reader.current_char {
        Some('\r') => {
            char_reader.read_and_stash_char();
            match char_reader.current_char {
                Some('\n') => {
                    char_reader.read_and_stash_char();
                    return Ok(~Newline(TokenData {
                        string: ~"\r\n",
                        row: 0,
                        col: 0
                    }));
                },
                _ => {
                    return Ok(~Newline(TokenData {
                        string: ~"\r",
                        row: 0,
                        col: 0
                    }));
                }
            }
        },
        _ => {
            char_reader.read_and_stash_char();
            return Ok(~Newline(TokenData {
                string: ~"\n",
                row: 0,
                col: 0
            }));
        }
    }
}

fn is_valid_string_start(char: char) -> bool {
    char == '\'' || char == '"'
}

fn string_tokenizer(char_reader: &mut CharReader) -> Result<~Token, ~str> {
    let mut token: ~[char] = ~[];
    let opening_character = char_reader.current_char.unwrap();
    char_reader.read_and_stash_char();

    loop {
        match char_reader.current_char {
            Some(char) if opening_character == char => {
                //Lookahead
                let backtrack = char_reader.create_backtrack();
                char_reader.read_and_stash_char();
                match char_reader.current_char {
                    Some(char) if opening_character == char => {
                        //It's a quote - continue
                        token.push(char);
                    },
                    _ => {
                        char_reader.backtrack(&backtrack);
                        return Ok(~String(TokenData {
                            string: str::from_chars(token),
                            row: 0,
                            col: 0
                        }));
                    }
                }
            },
            Some(char) => {
                token.push(char);
            },
            None => {
                return Err(~"Unexpected end of file");
            }
        };
        char_reader.read_and_stash_char();
    }
}

fn is_valid_variable_start(char: char) -> bool {
    char == '∆' || char == '⍙' || (char >= 'A' && char <= 'z')
}

fn variable_tokenizer(char_reader: &mut CharReader) -> Result<~Token, ~str> {
    let mut token: ~[char] = ~[];

    loop {
        match char_reader.current_char {
            Some(char) => {
                if is_valid_variable_start(char) {
                    token.push(char);
                } else {
                    break;
                }
            },
            None => {
                break;
            }
        };
        char_reader.read_and_stash_char();
    }
    return Ok(~Variable(TokenData {
        string: str::from_chars(token),
        row: 0,
        col: 0
    }));
}

fn is_dot(char: char) -> bool {
    char == '.'
}

fn dot_tokenizer(char_reader: &mut CharReader) -> Result<~Token, ~str> {
    let backtrack = char_reader.create_backtrack();
    char_reader.read_and_stash_char();
    match char_reader.current_char {
        Some(char) if char::is_digit(char) => {
            char_reader.backtrack(&backtrack);
            return number_tokenizer(char_reader);
        },
        _ => {
            Ok(~Primitive(TokenData {
                string: ~".",
                row: 0,
                col: 0
            }))
        }
    }
}

fn is_valid_primitive_start(char: char) -> bool {
    *(~['+','−','×','÷','⌈','⌊','∣','|','⍳','?','⋆','*','⍟','○','!','⌹','<','≤','=','≥','>','≠','≡','≢','∊','⍷','∪','∩','~','∨','∧','⍱','⍲','⍴',',','⍪','⌽','⊖','⍉','↑','↓','⊂','⊃','⌷','⍋','⍒','⊤','⊥','⍺','⍕','⍎','⊣','⊢','▯','⍞','/','\\','⍀','⌿','∘','¨','[',']','⍬','⋄','∇','⍫','(',')','←', '{', '}', '⍵', '-'].contains(&char))
}


fn primitive_tokenizer(char_reader: &mut CharReader) -> Result<~Token, ~str> {
    let opening_character = char_reader.current_char.unwrap();
    if opening_character == '∘' {
        let backtrack = char_reader.create_backtrack();
        char_reader.read_and_stash_char();
        match char_reader.current_char {
            Some('.') => {
                char_reader.read_and_stash_char();
                Ok(~Primitive(TokenData {
                    string: ~"∘.",
                    row: 0,
                    col: 0
                }))
            },
            _ => {
                char_reader.backtrack(&backtrack);
                Ok(~Primitive(TokenData {
                    string: ~"∘",
                    row: 0,
                    col: 0
                }))
            }
        }
    } else {
        char_reader.read_and_stash_char();
        Ok(~Primitive(TokenData {
            string: str::from_char(opening_character),
            row: 0,
            col: 0
        }))
    }
}
