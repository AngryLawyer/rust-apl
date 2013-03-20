use tokenizer;
use tokenizer::Token;

pub enum Type {
    Number(~Token),
    Array(~[Token]),
    Identity(~Token)
}

pub enum Node {
    pub Niladic(~Type),
    pub Monadic(~Type, ~Node),
    pub Dyadic(~Type, ~Node, ~Node),
    pub Sequence(~[~Node])
}

fn try_parse_dyadic(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    result::Err(~"NOPE")
}

fn try_parse_monadic(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    result::Err(~"NOPE")
}

fn try_parse_array(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    result::Err(~"NOPE")
}

fn try_parse_number(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {
    result::Err(~"NOPE")
}

fn try_parse(tokenizer: @mut tokenizer::Tokenizer) -> result::Result<~Node, ~str> {

    let mut error = ~"";

    for [try_parse_dyadic, try_parse_monadic, try_parse_array, try_parse_number].each |&f| {
        match f(tokenizer) {
            result::Ok(node) => {
                return result::Ok(node);
            },
            result::Err(str) => {
                error = str;
            }
        }
    }
    result::Err(error)
}

pub struct Parser {
    tokenizer: @mut tokenizer::Tokenizer,
}

impl Parser {

    pub fn new(input_string: ~str) -> ~Parser {
        ~Parser {
            tokenizer: @mut tokenizer::Tokenizer::new(input_string) 
        }
    }

    pub fn parse(&mut self) -> result::Result<~Node, ~str> {

        let mut sequence: ~[~Node] = ~[];

        loop {
            match self.tokenizer.read_next_token() {
                result::Ok(~tokenizer::EndOfFile) => {
                    break;
                },
                result::Err(msg) => {
                    return result::Err(msg);
                },
                result::Ok(token) => {
                    match try_parse(self.tokenizer) {
                        result::Ok(node) => {
                            sequence.push(node);
                        },
                        result::Err(msg) => {
                            return result::Err(msg);
                        }
                    }
                }
                
            }
        }

        result::Ok(~Sequence(sequence))
    }
}
