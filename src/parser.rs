use tokenizer;
use tokenizer::Token;
use core::cell;

pub enum Type {
    Number(~Token),
    Array(~[Token])
}

pub enum Node {
    pub Niladic(~Type),
    pub Monadic(~Type, ~Node),
    pub Dyadic(~Type, ~Node, ~Node)
}

pub struct Parser {
    tokenizer: ~tokenizer::Tokenizer,
}

impl Parser {

    pub fn new(input_string: ~str) -> ~Parser {
        ~Parser {
            tokenizer: tokenizer::Tokenizer::new(input_string)
        }
    }

    pub fn parse(&mut self) -> result::Result<~Node, ~str> {

        let mut parse_tree: @cell::Cell<~Node> = @cell::empty_cell();

        loop {
            match self.tokenizer.read_next_token() {
                result::Ok(~tokenizer::EndOfFile) => {
                    break;
                },
                result::Err(msg) => {
                    return result::Err(msg);
                },
                result::Ok(token) => {
                    break;
                }
                
            }
        }

        if parse_tree.is_empty() {
            result::Err(~"Unexpected end of file")
        } else {
            result::Ok(parse_tree.take())
        }
    }
}
