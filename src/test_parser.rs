use parser;
use parser::Parser;

#[test]
fn test_parse_number() {
   
    let number = ~"3.141"; //Everyone's favourite number 
    let mut parser = Parser::new(number);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            match tree {
                ~parser::Array(_) => {
                    //OK
                },
                _ => {
                    fail!(~"Didn't find a number");
                }
            }
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }
}

#[test]
fn test_parse_array() {

    let numbers = ~"1 2 3 4";
    let mut parser = Parser::new(numbers);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            match tree {
                ~parser::Array(numbers) => {
                    fail_unless!(numbers.len() == 4);
                },
                _ => {
                    fail!(~"Didn't find an array");
                }
            }
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }
}

#[test]
fn test_parse_variable() {
   
    let var = ~"Trololo";
    let mut parser = Parser::new(var);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            match tree {
                ~parser::Variable(_) => {
                    //OK
                },
                _ => {
                    fail!(~"Didn't find a variable");
                }
            }
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }
}

#[test]
fn test_parse_zilde() {
   
    let var = ~"⍬";
    let mut parser = Parser::new(var);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            match tree {
                ~parser::Zilde(_) => {
                    //OK
                },
                _ => {
                    fail!(~"Didn't find zilde");
                }
            }
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }
}

#[test]
fn test_conjugate() {
    let expression = ~"+1";
    let mut parser = Parser::new(expression);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            match tree {
                ~parser::Conjugate(_, ~parser::Array(_)) => {
                    //OK
                },
                _ => {
                    fail!(~"Didn't find the right Monadic expression");
                }
            }
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }

    let expression = ~"+";
    let mut parser = Parser::new(expression);
    match parser.parse_next_statement() {
        result::Err(msg) => {
        },
        _ => {
            fail!(~"Incorrectly parsed invalid expression");
        }
    }
}

#[test]
fn test_negate() {
    let expression = ~"−1";
    let mut parser = Parser::new(expression);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            match tree {
                ~parser::Negate(_, ~parser::Array(_)) => {
                    //OK
                },
                _ => {
                    fail!(~"Didn't find the right Monadic expression");
                }
            }
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }

    let expression = ~"-1";
    let mut parser = Parser::new(expression);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            match tree {
                ~parser::Negate(_, ~parser::Array(_)) => {
                    //OK
                },
                _ => {
                    fail!(~"Didn't find the right Monadic expression");
                }
            }
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }
}

#[test]
fn test_addition() {
    let expression = ~"1 2 3 4+2 4 6 8";
    let mut parser = Parser::new(expression);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            match tree {
                ~parser::Addition(_, ~parser::Array(_), ~parser::Array(_)) => {
                    //OK
                },
                _ => {
                    fail!(~"Didn't find the right Dyadic expression");
                }
            }
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }
}
