use parser;
use nodes;
use tokenizer;

pub trait Printable {
    pub fn to_string(&self) -> ~str;
}

pub enum Value {
    pub Float(float),
    pub Integer(int),
    pub Array(uint, ~[~Value])
}

impl Printable for Value {

    pub fn to_string(&self) -> ~str {
        match self {
            &Float(f) => {
                fmt!("%f", f)
            },
            &Integer(i) => {
                fmt!("%i", i)
            },
            &Array(depth, ref contents) => {
                if depth != 1 {
                    fail!(~"Multidimensional arrays aren't yet supported");
                }
                let mut result: ~str = ~"";
                let mut first: bool = true;

                for contents.each |value| {
                    if first {
                        first = false;
                    } else {
                        result += " ";
                    }
                    result += value.to_string()
                }
                result
            }/*,
            _ => {
                fail!(~"Unknown type")
            }*/
        }
    }
}

pub fn eval_node(node: &nodes::Node) -> result::Result<~Value,~str> {
    match node {
        &nodes::Array(ref nodes) => result::Ok(eval_array(nodes)),
        &nodes::Addition(_, _, _) => eval_addition(node),
        _ => result::Err(~"Not yet implemented")
    }
}

fn eval_addition(node: &nodes::Node) -> result::Result<~Value, ~str> {
    match node {
        &nodes::Addition(_, ref left, ref right) => {
            let eval_left = eval_node(*left);
            let eval_right = eval_node(*right);
            match (eval_left, eval_right) {
                (result::Ok(~Integer(x)), result::Ok(~Integer(y))) => { //FIXME: Should be returning errors here
                    result::Ok(~Integer(x+y))
                },
                _ => result::Err(~"Need to implement addition!")
            }
        },
        _ => result::Err(~"Trying to do addition on non-addition item")
    }
    
}

fn eval_array(tokens: &~[@tokenizer::Token]) -> ~Value {
    if tokens.len() == 1 {
        eval_number(tokens[0])
    } else {
        let mut array_contents: ~[~Value] = ~[];
        for tokens.each |token| {
            array_contents.push(eval_number(*token));
        }
        ~Array(1, array_contents)

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
                            ~Float(fl)
                        },
                        option::None => fail!(~"Bad float")
                    }
                },
                option::None => {
                    match int::from_str(token_data.string) {
                        option::Some(i) => {
                            ~Integer(i)
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
