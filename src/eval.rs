use parser;
use nodes;
use tokenizer;

trait Value {}

pub struct Float {
    value: float
}

impl Value for Float;

pub struct Integer {
    value: int
}

impl Value for Integer;

pub struct Array {
    dimensions: uint,
    values: ~[~Value]
}

impl Value for Array;

pub fn eval_node(node: &nodes::Node) -> result::Result<~Value,~str> {
    match node {
        &nodes::Array(ref nodes) => eval_array(nodes),
        _ => result::Err(~"Not yet implemented")
    }
}

fn eval_array(tokens: &~[@tokenizer::Token]) -> result::Result<~Value, ~str> {
    if tokens.len() == 1 {
        result::Ok(eval_number(tokens[0]))
    } else {
        result::Err(~"Can't deal with multidimensional arrays")
    }

}

fn eval_number(token: @tokenizer::Token) -> ~Value {
    match token {
        @tokenizer::Number(ref token_data) => { 
            //FIXME: This needs to handle exponents and complex numbers
            match str::find_char(token_data.string, '.') {
                option::Some(_) => {
                    match float::from_str(token_data.string) {
                        option::Some(fl) => {
                            ~Float{value: fl} as ~Value
                        },
                        option::None => fail!(~"Bad float")
                    }
                },
                option::None => {
                    match int::from_str(token_data.string) {
                        option::Some(i) => {
                            ~Integer{value: i} as ~Value
                        },
                        option::None => fail!(~"Bad int")
                    }
                }
            }
        },
        _ => fail!(~"Something is seriously wrong")
    }
    
}

pub struct Evaluator {
    parser: @mut parser::Parser
}

pub impl Evaluator {
    
    pub fn new(input_string: ~str) -> Evaluator {
        Evaluator {
            parser: @mut parser::Parser::new(input_string)
        }
    }

    pub fn eval(&mut self) -> result::Result<~Value, ~str> {
        let tree = self.parser.parse_next_statement(); //TODO: Should loop?
        match tree {
            result::Ok(node) => {
                eval_node(node)
            },
            result::Err(msg) => {
                result::Err(msg)
            }
        }
    }
}
