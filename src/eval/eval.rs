use std::str;
use std::from_str::from_str;

use parser;
use nodes;
use tokenizer;
use nodes::EvalNode;

pub trait Printable {
    fn to_string(&self) -> ~str;
    fn to_typed_string(&self) -> ~str;
}

#[deriving(Eq)]
pub enum Value {
    AplFloat(f64),
    AplInteger(int),
    AplComplex(~Value, ~Value),
    AplArray(uint, ~[uint], ~[~Value])
}

impl Clone for Value {
    fn clone(&self) -> Value {
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

    fn to_string(&self) -> ~str {
        match self {
            &AplFloat(f) => {
                format!("{}", f)
            },
            &AplInteger(i) => {
                format!("{}", i)
            },
            &AplArray(depth, ref _dimensions, ref contents) => {
                if depth != 1 {
                    fail!(~"Multidimensional arrays aren't yet supported");
                }
                let segments: ~[~str] = contents.iter().map(|item| item.to_string()).collect();

                segments.connect(" ")
            },
            &AplComplex(ref left, ref right) => {
                left.to_string().append("J").append(right.to_string())
            }
        }
    }

    fn to_typed_string(&self) -> ~str {
        match self {
            &AplFloat(_) => {
                format!("FLOAT({})", self.to_string())
            },
            &AplInteger(_) => {
                format!("INTEGER({})", self.to_string())
            },
            &AplArray(_, _, _) => {
                format!("ARRAY({})", self.to_string())
            },
            &AplComplex(_, _) => {
                format!("COMPLEX({})", self.to_string())
            }
        }
    }
}

pub fn eval_node(node: &nodes::Node) -> Result<~Value,~str> {
    match node {
        &nodes::Array(ref nodes) => Ok(eval_array(nodes)),
        _ => node.eval()
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
        Some(pos) => {
            eval_complex(token_string.slice_to(pos), token_string.slice_from(pos + 1))
        },
        None => {
            match token_string.find('.') {
                Some(_) => {
                    eval_float(token_string)
                },
                None => {
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

    match from_str::<f64>(match_string) {
        Some(fl) => {
            if is_negative {
                ~AplFloat(-fl)
            } else {
                ~AplFloat(fl)
            }
        },
        None => {
            fail!(format!("Bad float {}", token_string))
        }
    }
}

fn eval_int(token_string: &str) -> ~Value {
    let (match_string, is_negative) = get_string_and_sign(token_string);

    match from_str::<int>(match_string) {
        Some(i) => {
            if is_negative {
                ~AplInteger(-i)
            } else {
                ~AplInteger(i)
            }
        },
        None => {
            fail!(format!("Bad int {}", token_string))
        }
    }
}

pub fn eval_dyadic(func: extern fn(&Value, &Value) -> Result<~Value, ~str>, left: &nodes::Node, right: &nodes::Node) -> Result<~Value, ~str> {
    match eval_node(left) {
        Ok(left) => {
            match eval_node(right) {
                Ok(right) => {
                    func(left, right)
                },
                Err(msg) => {
                    Err(msg)
                }
            }
        },
        Err(msg) => {
            Err(msg)
        }
    }
}

pub fn eval_monadic(func: extern fn(&Value) -> Result<~Value, ~str>, left: &nodes::Node) -> Result<~Value, ~str> {
    eval_node(left).and_then(|result| {
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

    pub fn eval(&mut self) -> Result<~Value, ~str> {
        let tree = self.parser.parse_next_statement(); //TODO: Should loop?
        match tree {
            Ok(node) => {
                eval_node(node)
            },
            Err(msg) => {
                Err(msg)
            }
        }
    }
}
