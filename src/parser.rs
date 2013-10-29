use tokenizer;
use tokenizer::Token;
use nodes::{Zilde, Variable, Node, Array, Parseable};

pub struct Parser {
    tokenizer: ~tokenizer::Tokenizer,
    current_token: Option<~tokenizer::Token>
}

impl Parser {

    pub fn new(input_string: ~str) -> Parser {
        Parser {
            tokenizer: ~tokenizer::Tokenizer::new(input_string),
            current_token: None
        }
    }

    pub fn parse_next_statement(&mut self) -> Result<~Node, ~str> {

        match self.read_next_token() {
            Ok(()) => {
                match self.current_token {
                    Some(~tokenizer::EndOfFile) => {
                        Err(~"End of File")
                    },
                    Some(_) => {
                        self.parse_dyadic()
                    },
                    None => {
                        Err(~"Everything is wrong")
                    }
                }
            },
            Err(msg) => {
                Err(msg)
            }
        }
    }

    fn read_next_token(&mut self) -> Result<(), ~str> {
        match self.tokenizer.read_next_token() {
            Ok(token) => {
                self.current_token = Some(token);
                Ok(())
            },
            Err(msg) => {
                self.current_token = None;
                Err(msg)
            }
        }
    }

    fn end_of_source(&self) -> bool {
        match self.current_token {
            None => true,
            Some(~tokenizer::EndOfFile) => true,
            _ => false
        }
    }

    fn token_is_number(&self) -> bool {
        match self.current_token {
            Some(~tokenizer::Number(_)) => true,
            _ => false
        }
    }

    pub fn create_dyadic_result(&mut self, left: ~Node, kind: &fn(~Token, ~Node, ~Node) -> Node) -> Result<~Node, ~str> {
        let stash = self.stash();
        match self.parse_dyadic() {
            Ok(node) => {
                let item = ~kind(stash, left, node);
                self.read_next_token();
                Ok(item)
            },
            Err(msg) => {
                Err(msg)
            }
        }
    }

    fn parse_dyadic(&mut self) -> Result<~Node, ~str> {
        if self.end_of_source() {
            Err(~"Unexpected end of source")
        } else {
            //Parse monadic on the left (otherwise it's an endless loop).
            match self.parse_monadic() {
                Ok(left) => {
                    if self.end_of_source() {
                        Ok(left)
                    } else {
                        match self.current_token {
                            Some(~tokenizer::Primitive(ref token_data)) => {
                                token_data.dyadic(self, left)
                            },
                            _ => {
                                Ok(left)
                            }
                        }
                    }
                },
                Err(msg) => Err(msg)
            }
        }
    }

    fn stash(&mut self) -> ~tokenizer::Token {
        let stash = self.current_token.unwrap();
        self.read_next_token();
        stash
    }

    pub fn create_monadic_result(&mut self, kind: &fn(~Token, ~Node) -> Node) -> Result<~Node, ~str> {
        let stash = self.stash();
        match self.parse_dyadic() {
            Ok(node) => {
                let item = ~kind(stash, node);
                self.read_next_token();
                Ok(item)
            },
            Err(msg) => {
                Err(msg)
            }
        }
    }

    fn parse_monadic(&mut self) -> Result<~Node, ~str> {
        if self.end_of_source() {
            Err(~"Unexpected end of source")
        } else {
            match self.current_token {
                Some(~tokenizer::Primitive(ref token_data)) => {
                    token_data.monadic(self)
                },
                _ => self.parse_base_expression()
            }
        }
    }

    pub fn parse_base_expression(&mut self) -> Result<~Node, ~str> {
        //This will either be an Array, a Number, or a Niladic primitive (or a bracketed thingy)
        if self.end_of_source() {
            Err(~"Unexpected end of source")
        } else {
            //FIXME: Better error handling
            match self.current_token {
                Some(~tokenizer::Number(_)) => self.parse_array(),
                Some(~tokenizer::Variable(_)) => self.parse_variable(),
                Some(~tokenizer::Primitive(ref token_data)) => {
                    match token_data.string {
                        ~"⍬" => self.parse_zilde(),
                        ~"(" => Err(~"Not yet implemented"),
                        _ => Err(~"Unexpected primitive")
                    }
                },
                //TODO: 
                _ => Err(~"Unexpected token")
            }
        }
    }

    fn parse_array(&mut self) -> Result<~Node, ~str> {
        let mut tokens: ~[~Token] = ~[];
        while self.token_is_number() {
            tokens.push(self.current_token.unwrap());
            self.read_next_token();
        }
        Ok(~Array(tokens))
    }

    fn parse_variable(&mut self) -> Result<~Node, ~str> {
        let result = ~Variable(self.current_token.unwrap());
        self.read_next_token();
        Ok(result)
    }

    fn parse_zilde(&mut self) -> Result<~Node, ~str> {
        let result = ~Zilde(self.current_token.unwrap());
        self.read_next_token();
        Ok(result)
    }

    /*fn skip_expected<T>(&mut self, token_string: &str) -> Result<(), ~str> { //FIXME: This should type check
        if self.end_of_source() {
            Err(~"Unexpected end of source")
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
