use tokenizer;
use tokenizer::Token;

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

    static fn new(input_string: ~str) -> ~Parser {
        ~Parser {
            tokenizer: tokenizer::Tokenizer::new(input_string)
        }
    }

    pub fn parse(&mut self) -> result::Result<~Node, ~str> {

        let mut parse_tree: option::Option<~Node> = option::None;

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
        match parse_tree {
            option::Some(node) => {
                result::Ok(node)
            },
            option::None => {
                result::Err(~"Unexpected end of file")
            }
        }
    }
}
