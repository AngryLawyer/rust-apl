use std::{result, option, int, float, str};

use parser;
use nodes;
use tokenizer;

pub trait Printable {
    pub fn to_string(&self) -> ~str;
}

pub enum Value {
    pub Float(float),
    pub Integer(int),
    pub Complex(~Value, ~Value),
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

                for contents.iter().advance |value| {
                    if first {
                        first = false;
                    } else {
                        result = result.append(" ");
                    }
                    result = result.append(value.to_string())
                }
                result
            },
            &Complex(ref left, ref right) => {
                left.to_string().append("J").append(right.to_string())
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
        &nodes::Addition(_, ref left, ref right) => eval_addition(*left, *right),
        _ => result::Err(~"Not yet implemented")
    }
}

fn eval_addition(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    match eval_node(left) {
        result::Ok(left) => {
            match eval_node(right) {
                result::Ok(right) => {
                    match (left, right) {
                        (~Integer(x), ~Integer(y)) => {
                            result::Ok(~Integer(x+y))
                        },
                        (~Float(x), ~Float(y)) => {
                            result::Ok(~Float(x+y))
                        },
                        (~Integer(x), ~Float(y)) => {
                            result::Ok(~Float(x as float + y))
                        },
                        (~Float(x), ~Integer(y)) => {
                            result::Ok(~Float(x + y as float))
                        },
                        _ => result::Err(~"Need to implement addition for arrays and complex!")
                    }
                },
                result::Err(msg) => {
                    result::Err(msg)
                }
            }
        },
        result::Err(msg) => {
            result::Err(msg)
        }
    }
}

fn eval_array(tokens: &~[@tokenizer::Token]) -> ~Value {
    if tokens.len() == 1 {
        match tokens[0] {
            @tokenizer::Number(ref token_data) => {
                eval_number(token_data.string)
            },
            _ => {
                fail!("Unsupported type in array")
            }
        }
    } else {
        let mut array_contents: ~[~Value] = ~[];
        for tokens.iter().advance|token| {
            match *token {
                @tokenizer::Number(ref token_data) => {
                    array_contents.push(eval_number(token_data.string))
                },
                _ => {
                    fail!("Unsupported type in array")
                }
            }
        }
        ~Array(1, array_contents)
    }
}

fn eval_number(token_string: &str) -> ~Value {
    match token_string.find('J') {
        //FIXME: This needs to handle exponents
        option::Some(pos) => {
            eval_complex(token_string.slice_to(pos), token_string.slice_from(pos + 1))
        },
        option::None => {
            match token_string.find('.') {
                option::Some(_) => {
                    eval_float(token_string)
                },
                option::None => {
                    eval_int(token_string)
                }
            }
        }
    }
}

fn get_string_and_sign<'r>(token_string: &'r str) -> (&'r str, bool){
    if token_string.char_at(0) == '¯' {
        let range: str::CharRange = token_string.char_range_at(0);
        (token_string.slice_from(range.next), true)
    } else {
        (token_string, false)
    }
}

fn eval_complex(left: &str, right: &str) -> ~Value {
    ~Complex(eval_number(left), eval_number(right))
}

fn eval_float(token_string: &str) -> ~Value {
    let (match_string, is_negative) = get_string_and_sign(token_string);

    match float::from_str(match_string) {
        option::Some(fl) => {
            if is_negative {
                ~Float(-fl)
            } else {
                ~Float(fl)
            }
        },
        option::None => {
            fail!(fmt!("Bad float %s", token_string))
        }
    }
}

fn eval_int(token_string: &str) -> ~Value {
    let (match_string, is_negative) = get_string_and_sign(token_string);

    match int::from_str(match_string) {
        option::Some(i) => {
            if is_negative {
                ~Integer(-i)
            } else {
                ~Integer(i)
            }
        },
        option::None => {
            fail!(fmt!("Bad int %s", token_string))
        }
    }
}

pub struct Evaluator {
    parser: @mut parser::Parser
}

impl Evaluator {
    
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
