pub struct TokenData {
    string: ~str,
    row: uint,
    col: uint
}

pub enum Token {
    pub Number(TokenData),
    pub Newline(TokenData),
    pub String(TokenData),
    pub Primitive(TokenData),
    pub Variable(TokenData),
    pub EndOfFile()
}

struct Backtrack {
    initial_next: uint,
    initial_char: option::Option<char>,
    initial_row: uint,
    initial_col: uint
}

struct CharReader {
    source: ~str,
    next: uint,
    current_char: option::Option<char>,
    row: uint,
    col: uint
}

pub fn print_token(token: Token) {
    match token {
        Number(data) => {
            io::println(fmt!("NUMBER: %s", data.string));
        },
        Newline(_data) => {
            io::println(~"NEWLINE");
        },
        String(data) => {
            io::println(fmt!("STRING: %s", data.string));
        },
        Primitive(data) => {
            io::println(fmt!("PRIMITIVE: %s", data.string));
        },
        Variable(data) => {
            io::println(fmt!("VARIABLE: %s", data.string));
        },
        EndOfFile => {
            io::println(~"EOF");
        }
    }
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
    char_reader: @mut CharReader,
}

impl Tokenizer {
    static fn new(input_string: ~str) -> Tokenizer {
        let mut char_reader = CharReader::new(input_string);
        char_reader.read_char();
        Tokenizer {
            char_reader: @mut char_reader
        }
    }

    pub fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        self.char_reader.wind_past_whitespace();
        self.char_reader.wind_past_comments();
        match self.char_reader.current_char {
            option::Some(first_char) => {
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
                result::Err(fmt!("No valid token found starting with %c" first_char))
            },
            option::None => {
                result::Ok(EndOfFile)
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
    backtrack: Backtrack
}

impl NewlineTokenizer {

    static fn is_valid_newline_start(char: char) -> bool {
        char == '\n' || char == '\r'
    }

    static fn new(char_reader: @mut CharReader) -> NewlineTokenizer {
        NewlineTokenizer {
            char_reader: char_reader,
            backtrack: char_reader.create_backtrack()
        }
    }

    fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        match self.char_reader.current_char {
            option::Some('\r') => {
                self.char_reader.read_char();
                match self.char_reader.current_char {
                    option::Some('\n') => {
                        self.char_reader.read_char();
                        return result::Ok(Newline(TokenData {
                            string: ~"\r\n",
                            row: 0,
                            col: 0
                        }));
                    },
                    _ => {
                        return result::Ok(Newline(TokenData {
                            string: ~"\r",
                            row: 0,
                            col: 0
                        }));
                    }
                }
            },
            _ => {
                self.char_reader.read_char();
                return result::Ok(Newline(TokenData {
                    string: ~"\n",
                    row: 0,
                    col: 0
                }));
            }
        }
    }
}

struct StringTokenizer {
    char_reader: @mut CharReader
}

impl StringTokenizer {

    static fn is_valid_string_start(char: char) -> bool {
        char == '\'' || char == '"'
    }

    static fn new(char_reader: @mut CharReader) -> StringTokenizer {
        StringTokenizer {
            char_reader: char_reader
        }
    }

    fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        let mut token: ~[char] = ~[];
        let opening_character = option::unwrap(self.char_reader.current_char);
        self.char_reader.read_char();

        loop {
            match self.char_reader.current_char {
                option::Some(char) if opening_character == char => {
                    //Lookahead
                    let backtrack = self.char_reader.create_backtrack();
                    self.char_reader.read_char();
                    match self.char_reader.current_char {
                        option::Some(char) if opening_character == char => {
                            //It's a quote - continue
                            token.push(char);
                        },
                        _ => {
                            self.char_reader.backtrack(&backtrack);
                            return result::Ok(String(TokenData {
                                string: str::from_chars(token),
                                row: 0,
                                col: 0
                            }));
                        }
                    }
                },
                option::Some(char) => {
                    token.push(char);
                },
                option::None => {
                    return result::Err(~"Unexpected end of file");
                }
            };
            self.char_reader.read_char();
        }
    }
}

struct VariableTokenizer {
    char_reader: @mut CharReader
}

impl VariableTokenizer {

    static fn is_valid_variable_start(char: char) -> bool {
        char == '∆' || char == '⍙' || (char >= 'A' && char <= 'z')
    }

    static fn new(char_reader: @mut CharReader) -> VariableTokenizer {
        VariableTokenizer {
            char_reader: char_reader
        }
    }

    fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        let mut token: ~[char] = ~[];

        loop {
            match self.char_reader.current_char {
                option::Some(char) => {
                    if VariableTokenizer::is_valid_variable_start(char) {
                        token.push(char);
                    } else {
                        break;
                    }
                },
                option::None => {
                    break;
                }
            };
            self.char_reader.read_char();
        }
        return result::Ok(Variable(TokenData {
            string: str::from_chars(token),
            row: 0,
            col: 0
        }));
    }
}

struct DotTokenizer {
    char_reader: @mut CharReader
}

impl DotTokenizer {

    static fn is_dot(char: char) -> bool {
        char == '.'
    }

    static fn new(char_reader: @mut CharReader) -> DotTokenizer {
        DotTokenizer {
            char_reader: char_reader
        }
    }

    fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        let backtrack = self.char_reader.create_backtrack();
        self.char_reader.read_char();
        match self.char_reader.current_char {
            option::Some(char) if char::is_digit(char) => {
                self.char_reader.backtrack(&backtrack);
                let mut tokenizer = NumberTokenizer::new(self.char_reader);
                return tokenizer.read_next_token()
            },
            _ => {
                result::Ok(Primitive(TokenData {
                    string: ~".",
                    row: 0,
                    col: 0
                }))
            }
        }
    }
}

struct PrimitiveTokenizer {
    char_reader: @mut CharReader
}

impl PrimitiveTokenizer {

    static fn is_valid_primitive_start(char: char) -> bool {
        vec::contains(~['+','−','×','÷','⌈','⌉','∣','⍳','?','⋆','⍟','○','!','⌹','<','≤','=','≥','>','≠','≡','≢','∊','⍷','∪','∩','~','∨','∧','⍱','⍲','⍴',',','⍪','⌽','⊖','⍉','↑','↓','⊂','⊃','⌷','⍋','⍒','⊤','⊥','⍺','⍕','⍎','⊣','⊢','▯','⍞','/','\\','⍀','⌿','∘','¨','[',']','⍬','⋄','∇','⍫','(',')','←', '{', '}', '⍵'], &char)
    }

    static fn new(char_reader: @mut CharReader) -> PrimitiveTokenizer {
        PrimitiveTokenizer {
            char_reader: char_reader
        }
    }

    fn read_next_token(&mut self) -> result::Result<Token, ~str> {
        let opening_character = option::unwrap(self.char_reader.current_char);
        if opening_character == '∘' {
            let backtrack = self.char_reader.create_backtrack();
            self.char_reader.read_char();
            match self.char_reader.current_char {
                option::Some('.') => {
                    self.char_reader.read_char();
                    result::Ok(Primitive(TokenData {
                        string: ~"∘.",
                        row: 0,
                        col: 0
                    }))
                },
                _ => {
                    self.char_reader.backtrack(&backtrack);
                    result::Ok(Primitive(TokenData {
                        string: ~"∘",
                        row: 0,
                        col: 0
                    }))
                }
            }
        } else {
            self.char_reader.read_char();
            result::Ok(Primitive(TokenData {
                string: str::from_char(opening_character),
                row: 0,
                col: 0
            }))
        }
    }

}
