use tokenizer;
use tokenizer::Token;

pub enum Type {
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
    //Also includes numbers, which are a zero-element array
    //TODO: Mark a rewind point, as we might be a Variable or niladic function
    //Read the first one. Is it alright? Read until we run out.
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
