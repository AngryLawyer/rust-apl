use tokenizer;
use tokenizer::Token;

pub enum Node {
    pub Array(~[@Token]),
    pub Identity(@Token, ~Node)
}

fn try_parse_dyadic(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    result::Err(~"NOPE")
}

fn try_parse_monadic(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    result::Err(~"NOPE")
}

fn try_parse_array(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    //An array is 
    //Also includes numbers, which are a zero-element array
    //TODO: Mark a rewind point, as we might be a Variable or niladic function
    //Read the first one. Is it alright? Read until we run out.
    result::Err(~"NOPE")
}

fn try_parse_number(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    //A number is an Array or an actual number
    result::Err(~"NOPE")
}

fn try_parse_base_expression(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    //This will either be an Array, a Number, or a Niladic primitive (or a bracketed thingy)

    result::Err(~"NOPE")
}

fn try_parse(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {

    //A sequence is any number of non-sequence items, until EOF is hit, I guess
    //Think about having Sequence as a linked list?
    //Loop
        //Try reading an expression
        //Expressions read until Line break or EOF, or that crazy diamond
    let mut error = ~"";
    result::Err(error)
}

pub struct Parser {
    tokenizer: @mut tokenizer::Tokenizer,
    current_token: option::Option<@tokenizer::Token>
}

impl Parser {

    pub fn new(input_string: ~str) -> ~Parser {
        ~Parser {
            tokenizer: @mut tokenizer::Tokenizer::new(input_string),
            current_token: option::None
        }
    }

    pub fn parse_next_statement(&mut self) -> result::Result<~Node, ~str> {

        match self.read_next_token() {
            result::Ok(()) => {
                match self.current_token {
                    option::Some(@tokenizer::EndOfFile) => {
                        result::Err(~"End of File")
                    },
                    option::Some(token) => {
                        self.parse_base_expression()
                    },
                    option::None => {
                        result::Err(~"Everything is wrong")
                    }
                }
            },
            result::Err(msg) => {
                result::Err(msg)
            }
        }
    }

    fn read_next_token(&mut self) -> result::Result<(), ~str> {
        match self.tokenizer.read_next_token() {
            result::Ok(token) => {
                self.current_token = option::Some(@token);
                result::Ok(())
            },
            result::Err(msg) => {
                self.current_token = option::None;
                result::Err(msg)
            }
        }
    }

    fn end_of_source(&self) -> bool {
        match self.current_token {
            option::None => true,
            option::Some(@tokenizer::EndOfFile) => true,
            _ => false
        }
    }

    fn token_is_number(&self) -> bool {
        match self.current_token {
            option::Some(@tokenizer::Number(_)) => true,
            _ => false
        }
    }

    /*fn clone_current_token(&self) -> Token {
        match self.current_token {
            option::Some(token) => token,
            _ => fail!(~"Tried to clone")
        }
    }*/

    fn parse_base_expression(&mut self) -> result::Result<~Node, ~str> {
        //This will either be an Array, a Number, or a Niladic primitive (or a bracketed thingy)
        if self.end_of_source() {
            result::Err(~"Unexpected end of source")
        } else {
            //FIXME: Better error handling
            if self.token_is_number() {
                self.parse_array()
            } else {
                result::Err(~"Unexpected token")
            }
        }
    }

    fn parse_array(&mut self) -> result::Result<~Node, ~str> {
        let mut tokens: ~[@Token] = ~[];
        while self.token_is_number() {
            tokens.push(option::get(self.current_token));
            self.read_next_token();
        }
        result::Ok(~Array(tokens))
    }

    /*fn skip_expected<T>(&mut self, token_string: &str) -> result::Result<(), ~str> { //FIXME: This should type check
        if self.end_of_source() {
            result::Err(~"Unexpected end of source")
        } else {
            match self.current_token {
                T(data) => {
                    data.string == token_string
                },
                _ => fail!(~"Should never reach here!")
            }
        }
    }*/
}
