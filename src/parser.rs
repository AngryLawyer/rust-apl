use std::{result, option};
use tokenizer;
use tokenizer::Token;
use nodes::*;

pub struct Parser {
    tokenizer: @mut tokenizer::Tokenizer,
    current_token: option::Option<@tokenizer::Token>
}

impl Parser {

    pub fn new(input_string: ~str) -> Parser {
        Parser {
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
                    option::Some(_token) => {
                        self.parse_dyadic()
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

    fn create_dyadic_result(&mut self, left: ~Node, kind: &fn(@Token, ~Node, ~Node) -> Node) -> result::Result<~Node, ~str> {
        let stash = self.stash();
        match self.parse_dyadic() {
            result::Ok(node) => {
                let item = ~kind(stash, left, node);
                self.read_next_token();
                result::Ok(item)
            },
            result::Err(msg) => {
                result::Err(msg)
            }
        }
    }

    fn parse_dyadic(&mut self) -> result::Result<~Node, ~str> {
        if self.end_of_source() {
            result::Err(~"Unexpected end of source")
        } else {
            //Parse monadic on the left (otherwise it's an endless loop).
            match self.parse_monadic() {
                result::Ok(left) => {
                    if self.end_of_source() {
                        result::Ok(left)
                    } else {
                        match self.current_token {
                            option::Some(@tokenizer::Primitive(ref token_data)) => {
                                match token_data.string {
                                    ~"+" => self.create_dyadic_result(left, Addition),
                                    ~"-" | ~"−" => self.create_dyadic_result(left, Subtraction),
                                    ~"×" => self.create_dyadic_result(left, Multiplication),
                                    ~"÷" => self.create_dyadic_result(left, Division),
                                    ~"⌈" => self.create_dyadic_result(left, Maximum),
                                    _ => result::Err(~"Unknown operator")
                                }
                            },
                            _ => {
                                result::Ok(left)
                            }
                        }
                    }
                },
                result::Err(msg) => result::Err(msg)
            }
        }
    }

    fn stash(&mut self) -> @tokenizer::Token {
        let stash = self.current_token.unwrap();
        self.read_next_token();
        stash
    }

    fn create_monadic_result(&mut self, kind: &fn(@Token, ~Node) -> Node) -> result::Result<~Node, ~str> {
        let stash = self.stash();
        match self.parse_dyadic() {
            result::Ok(node) => {
                let item = ~kind(stash, node);
                self.read_next_token();
                result::Ok(item)
            },
            result::Err(msg) => {
                result::Err(msg)
            }
        }
    }

    fn parse_monadic(&mut self) -> result::Result<~Node, ~str> {
        if self.end_of_source() {
            result::Err(~"Unexpected end of source")
        } else {
            match self.current_token {
                option::Some(@tokenizer::Primitive(ref token_data)) => {
                    match token_data.string {
                        ~"+" => self.create_monadic_result(Conjugate),
                        ~"-" | ~"−" => self.create_monadic_result(Negate),
                        ~"×" => self.create_monadic_result(Sign),
                        ~"÷" => self.create_monadic_result(Reciprocal),
                        ~"|" | ~"∣" => self.create_monadic_result(Magnitude),
                        ~"⌈" => self.create_monadic_result(Ceiling),
                        _ => self.parse_base_expression()
                    }
                },
                _ => self.parse_base_expression()
            }
        }
    }

    fn parse_base_expression(&mut self) -> result::Result<~Node, ~str> {
        //This will either be an Array, a Number, or a Niladic primitive (or a bracketed thingy)
        if self.end_of_source() {
            result::Err(~"Unexpected end of source")
        } else {
            //FIXME: Better error handling
            match self.current_token {
                option::Some(@tokenizer::Number(_)) => self.parse_array(),
                option::Some(@tokenizer::Variable(_)) => self.parse_variable(),
                option::Some(@tokenizer::Primitive(ref token_data)) => {
                    match token_data.string {
                        ~"⍬" => self.parse_zilde(),
                        ~"(" => result::Err(~"Not yet implemented"),
                        _ => result::Err(~"Unexpected primitive")
                    }
                },
                //TODO: 
                _ => result::Err(~"Unexpected token")
            }
        }
    }

    fn parse_array(&mut self) -> result::Result<~Node, ~str> {
        let mut tokens: ~[@Token] = ~[];
        while self.token_is_number() {
            tokens.push(self.current_token.unwrap());
            self.read_next_token();
        }
        result::Ok(~Array(tokens))
    }

    fn parse_variable(&mut self) -> result::Result<~Node, ~str> {
        let result = ~Variable(self.current_token.unwrap());
        self.read_next_token();
        result::Ok(result)
    }

    fn parse_zilde(&mut self) -> result::Result<~Node, ~str> {
        let result = ~Zilde(self.current_token.unwrap());
        self.read_next_token();
        result::Ok(result)
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
