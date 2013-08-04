use std::{result, option, int, str, float};

use parser;
use nodes;
use tokenizer;
use eval::add::eval_addition;
use eval::subtract::eval_subtraction;
use eval::multiply::eval_multiplication;
use eval::divide::eval_division;
use eval::conjugate::eval_conjugate;
use eval::negate::eval_negate;
use eval::reciprocal::eval_reciprocal;
use eval::sign::eval_sign;

pub trait Printable {
    pub fn to_string(&self) -> ~str;
    pub fn to_typed_string(&self) -> ~str;
}

#[deriving(Eq)]
pub enum Value {
    pub AplFloat(float),
    pub AplInteger(int),
    pub AplComplex(~Value, ~Value),
    pub AplArray(uint, ~[uint], ~[~Value])
}

impl Clone for Value {
    pub fn clone(&self) -> Value {
        match *self {
            AplFloat(f) => {
                AplFloat(f)
            },
            AplInteger(i) => {
                AplInteger(i)
            },
            AplComplex(ref a, ref b) => {
                AplComplex(a.clone(), b.clone())
            },
            AplArray(depth, ref dimensions, ref contents) => {
                AplArray(depth, dimensions.clone(), contents.clone())
            }
        }
    }
}

impl Printable for Value {

    pub fn to_string(&self) -> ~str {
        match self {
            &AplFloat(f) => {
                fmt!("%f", f)
            },
            &AplInteger(i) => {
                fmt!("%i", i)
            },
            &AplArray(depth, ref _dimensions, ref contents) => {
                if depth != 1 {
                    fail!(~"Multidimensional arrays aren't yet supported");
                }
                let segments: ~[~str] = contents.iter().transform(|item| item.to_string()).collect();

                segments.connect(" ")
            },
            &AplComplex(ref left, ref right) => {
                left.to_string().append("J").append(right.to_string())
            }
        }
    }

    pub fn to_typed_string(&self) -> ~str {
        match self {
            &AplFloat(_) => {
                fmt!("FLOAT(%s)", self.to_string())
            },
            &AplInteger(_) => {
                fmt!("INTEGER(%s)", self.to_string())
            },
            &AplArray(_, _, _) => {
                fmt!("ARRAY(%s)", self.to_string())
            },
            &AplComplex(_, _) => {
                fmt!("COMPLEX(%s)", self.to_string())
            }
        }
    }
}

pub fn eval_node(node: &nodes::Node) -> result::Result<~Value,~str> {
    match node {
        &nodes::Array(ref nodes) => result::Ok(eval_array(nodes)),
        &nodes::Addition(_, ref left, ref right) => eval_addition(*left, *right),
        &nodes::Subtraction(_, ref left, ref right) => eval_subtraction(*left, *right),
        &nodes::Multiplication(_, ref left, ref right) => eval_multiplication(*left, *right),
        &nodes::Division(_, ref left, ref right) => eval_division(*left, *right),

        &nodes::Conjugate(_, ref left) => eval_conjugate(*left),
        &nodes::Negate(_, ref left) => eval_negate(*left),
        &nodes::Reciprocal(_, ref left) => eval_reciprocal(*left),
        &nodes::Sign(_, ref left) => eval_sign(*left),

        _ => result::Err(~"Not yet implemented")
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
        for token in tokens.iter() {
            match *token {
                @tokenizer::Number(ref token_data) => {
                    array_contents.push(eval_number(token_data.string))
                },
                _ => {
                    fail!("Unsupported type in array")
                }
            }
        }
        ~AplArray(1, ~[array_contents.len()], array_contents)
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
    ~AplComplex(eval_number(left), eval_number(right))
}

fn eval_float(token_string: &str) -> ~Value {
    let (match_string, is_negative) = get_string_and_sign(token_string);

    match float::from_str(match_string) {
        option::Some(fl) => {
            if is_negative {
                ~AplFloat(-fl)
            } else {
                ~AplFloat(fl)
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
                ~AplInteger(-i)
            } else {
                ~AplInteger(i)
            }
        },
        option::None => {
            fail!(fmt!("Bad int %s", token_string))
        }
    }
}

pub fn eval_dyadic(func: extern fn(&Value, &Value) -> result::Result<~Value, ~str>, left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    match eval_node(left) {
        result::Ok(left) => {
            match eval_node(right) {
                result::Ok(right) => {
                    func(left, right)
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

pub fn eval_monadic(func: extern fn(&Value) -> result::Result<~Value, ~str>, left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_node(left).chain(|result| {
        func(result)
    })
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
