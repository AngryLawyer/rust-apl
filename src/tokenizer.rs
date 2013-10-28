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

    pub fn read_next_token(&mut self) -> Result<Token, ~str> {
        self.char_reader.wind_past_whitespace();
        self.char_reader.wind_past_comments();
        match self.char_reader.current_char {
            Some(first_char) => {
                //FIXME: Make Tokenizer a trait, turn this into two lines of code!
                if NewlineTokenizer::is_valid_newline_start(first_char) {
                    let mut tokenizer = NewlineTokenizer::new(self.char_reader);
                    return tokenizer.read_next_token()
                }
                if DotTokenizer::is_dot(first_char) {
                    let mut tokenizer = DotTokenizer::new(self.char_reader);
                    return tokenizer.read_next_token()
                }
                if NumberTokenizer::is_valid_number_start(first_char) {
                    let mut tokenizer = NumberTokenizer::new(self.char_reader);
                    return tokenizer.read_next_token()
                }
                if StringTokenizer::is_valid_string_start(first_char) {
                    let mut tokenizer = StringTokenizer::new(self.char_reader);
                    return tokenizer.read_next_token()
                }
                if PrimitiveTokenizer::is_valid_primitive_start(first_char) {
                    let mut tokenizer = PrimitiveTokenizer::new(self.char_reader);
                    return tokenizer.read_next_token()
                }
                if VariableTokenizer::is_valid_variable_start(first_char) {
                    let mut tokenizer = VariableTokenizer::new(self.char_reader);
                    return tokenizer.read_next_token()
                }
                Err(format!("No valid token found starting with {}", first_char))
            },
            None => {
                Ok(EndOfFile)
            }
        }
    }
}

struct NumberTokenizer {
    char_reader: &CharReader,
    period_encountered: bool,
    complex_encountered: bool,
    first_character: bool,
    allowed_negative: bool
}

impl NumberTokenizer {

    fn is_valid_number_start(char: char) -> bool {
        //Needs to be either upper dash, period, or 0-9
        (char >= '0' && char <= '9') || char == '.' || char == '¯'
    }

    fn new(char_reader: &CharReader) -> NumberTokenizer {
        NumberTokenizer {
            char_reader: char_reader,
            period_encountered: false,
            complex_encountered: false,
            first_character: true,
            allowed_negative: false
        }
    }

    fn is_period(&self) -> bool {
        match self.char_reader.current_char {
            Some('.') => true,
            _ => false
        }
    }

    fn is_number(&self) -> bool {
        match self.char_reader.current_char {
            Some(maybe_number) => {
                maybe_number >= '0' && maybe_number <= '9'
            },
            _ => false
        }
    }

    fn is_complex(&self) -> bool {
        match self.char_reader.current_char {
            Some('J') => true,
            _ => false
        }
    }

    fn is_negative(&self) -> bool {
        match self.char_reader.current_char {
            Some('¯') => true,
            _ => false
        }
    }

    fn read_next_token(&mut self) -> Result<Token, ~str> {
        let mut token: ~[char] = ~[];
        loop {
            if self.first_character {
                self.first_character = false;
                if self.is_period() {
                    self.period_encountered = true;
                }
                token.push(self.char_reader.current_char.unwrap());
            } else if self.is_negative() {
                if self.allowed_negative {
                    self.allowed_negative = false;
                    token.push(self.char_reader.current_char.unwrap());
                } else {
                    return Err(~"Invalid number");
                }
            } else if self.is_complex() {
                if self.complex_encountered {
                    return Err(~"Invalid number");
                } else {
                    self.complex_encountered = true;
                    self.period_encountered = false;
                    self.allowed_negative = true;
                    token.push(self.char_reader.current_char.unwrap());
                }
            } else if self.is_period() {
                self.allowed_negative = false;
                if self.period_encountered {
                    return Err(~"Invalid number");
                } else {
                    self.period_encountered = true;
                    token.push(self.char_reader.current_char.unwrap());
                }
            } else if self.is_number() {
                self.allowed_negative = false;
                token.push(self.char_reader.current_char.unwrap());
            } else {
                if (token[token.len() - 1] == '.' ||
                    token[token.len() - 1] == 'J' ||
                    token[token.len() - 1] == '¯') {
                    return Err(~"Invalid number");
                }
                return Ok(Number(TokenData {
                    string: str::from_chars(token),
                    row: 0,
                    col: 0
                }));
            }
            self.char_reader.read_and_stash_char();
        }
    }
}

struct NewlineTokenizer {
    char_reader: &CharReader,
    backtrack: Backtrack
}

impl NewlineTokenizer {

    fn is_valid_newline_start(char: char) -> bool {
        char == '\n' || char == '\r'
    }

    fn new(char_reader: &CharReader) -> NewlineTokenizer {
        NewlineTokenizer {
            char_reader: char_reader,
            backtrack: char_reader.create_backtrack()
        }
    }

    fn read_next_token(&mut self) -> Result<Token, ~str> {
        match self.char_reader.current_char {
            Some('\r') => {
                self.char_reader.read_and_stash_char();
                match self.char_reader.current_char {
                    Some('\n') => {
                        self.char_reader.read_and_stash_char();
                        return Ok(Newline(TokenData {
                            string: ~"\r\n",
                            row: 0,
                            col: 0
                        }));
                    },
                    _ => {
                        return Ok(Newline(TokenData {
                            string: ~"\r",
                            row: 0,
                            col: 0
                        }));
                    }
                }
            },
            _ => {
                self.char_reader.read_and_stash_char();
                return Ok(Newline(TokenData {
                    string: ~"\n",
                    row: 0,
                    col: 0
                }));
            }
        }
    }
}

struct StringTokenizer {
    char_reader: &CharReader
}

impl StringTokenizer {

    fn is_valid_string_start(char: char) -> bool {
        char == '\'' || char == '"'
    }

    fn new(char_reader: &CharReader) -> StringTokenizer {
        StringTokenizer {
            char_reader: char_reader
        }
    }

    fn read_next_token(&mut self) -> Result<Token, ~str> {
        let mut token: ~[char] = ~[];
        let opening_character = self.char_reader.current_char.unwrap();
        self.char_reader.read_and_stash_char();

        loop {
            match self.char_reader.current_char {
                Some(char) if opening_character == char => {
                    //Lookahead
                    let backtrack = self.char_reader.create_backtrack();
                    self.char_reader.read_and_stash_char();
                    match self.char_reader.current_char {
                        Some(char) if opening_character == char => {
                            //It's a quote - continue
                            token.push(char);
                        },
                        _ => {
                            self.char_reader.backtrack(&backtrack);
                            return Ok(String(TokenData {
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
            self.char_reader.read_and_stash_char();
        }
    }
}

struct VariableTokenizer {
    char_reader: &CharReader
}

impl VariableTokenizer {

    fn is_valid_variable_start(char: char) -> bool {
        char == '∆' || char == '⍙' || (char >= 'A' && char <= 'z')
    }

    fn new(char_reader: &CharReader) -> VariableTokenizer {
        VariableTokenizer {
            char_reader: char_reader
        }
    }

    fn read_next_token(&mut self) -> Result<Token, ~str> {
        let mut token: ~[char] = ~[];

        loop {
            match self.char_reader.current_char {
                Some(char) => {
                    if VariableTokenizer::is_valid_variable_start(char) {
                        token.push(char);
                    } else {
                        break;
                    }
                },
                None => {
                    break;
                }
            };
            self.char_reader.read_and_stash_char();
        }
        return Ok(Variable(TokenData {
            string: str::from_chars(token),
            row: 0,
            col: 0
        }));
    }
}

struct DotTokenizer {
    char_reader: &CharReader
}

impl DotTokenizer {

    fn is_dot(char: char) -> bool {
        char == '.'
    }

    fn new(char_reader: &CharReader) -> DotTokenizer {
        DotTokenizer {
            char_reader: char_reader
        }
    }

    fn read_next_token(&mut self) -> Result<Token, ~str> {
        let backtrack = self.char_reader.create_backtrack();
        self.char_reader.read_and_stash_char();
        match self.char_reader.current_char {
            Some(char) if char::is_digit(char) => {
                self.char_reader.backtrack(&backtrack);
                let mut tokenizer = NumberTokenizer::new(self.char_reader);
                return tokenizer.read_next_token()
            },
            _ => {
                Ok(Primitive(TokenData {
                    string: ~".",
                    row: 0,
                    col: 0
                }))
            }
        }
    }
}

struct PrimitiveTokenizer {
    char_reader: &CharReader
}

impl PrimitiveTokenizer {

    fn is_valid_primitive_start(char: char) -> bool {
        *(~['+','−','×','÷','⌈','⌊','∣','|','⍳','?','⋆','*','⍟','○','!','⌹','<','≤','=','≥','>','≠','≡','≢','∊','⍷','∪','∩','~','∨','∧','⍱','⍲','⍴',',','⍪','⌽','⊖','⍉','↑','↓','⊂','⊃','⌷','⍋','⍒','⊤','⊥','⍺','⍕','⍎','⊣','⊢','▯','⍞','/','\\','⍀','⌿','∘','¨','[',']','⍬','⋄','∇','⍫','(',')','←', '{', '}', '⍵', '-'].contains(&char))
    }

    fn new(char_reader: &CharReader) -> PrimitiveTokenizer {
        PrimitiveTokenizer {
            char_reader: char_reader
        }
    }

    fn read_next_token(&mut self) -> Result<Token, ~str> {
        let opening_character = self.char_reader.current_char.unwrap();
        if opening_character == '∘' {
            let backtrack = self.char_reader.create_backtrack();
            self.char_reader.read_and_stash_char();
            match self.char_reader.current_char {
                Some('.') => {
                    self.char_reader.read_and_stash_char();
                    Ok(Primitive(TokenData {
                        string: ~"∘.",
                        row: 0,
                        col: 0
                    }))
                },
                _ => {
                    self.char_reader.backtrack(&backtrack);
                    Ok(Primitive(TokenData {
                        string: ~"∘",
                        row: 0,
                        col: 0
                    }))
                }
            }
        } else {
            self.char_reader.read_and_stash_char();
            Ok(Primitive(TokenData {
                string: str::from_char(opening_character),
                row: 0,
                col: 0
            }))
        }
    }

}
