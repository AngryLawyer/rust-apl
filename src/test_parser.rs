use parser;
use parser::Parser;

fn first_of_sequence(parse_tree: ~parser::Node) -> ~parser::Node {
    match parse_tree {
        ~parser::Sequence(seqItems) => {
            fail_unless!(seqItems.len() == 1);
            copy seqItems[0]
        },
        _ => {
            fail!(~"Expected sequence")
        }
    }
}

#[test]
fn test_parse_number() {
   
    let number = ~"3.141"; //Everyone's favourite number 
    let mut parser = Parser::new(number);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            let item = first_of_sequence(tree);
            match item {
                ~parser::Niladic(parser::Array(_)) => {
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
            let item = first_of_sequence(tree);
            match item {
                ~parser::Niladic(parser::Array(numbers)) => {
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
fn test_monadic() {
    let expression = ~"+1";
    let mut parser = Parser::new(expression);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            let item = first_of_sequence(tree);
            match item {
                ~parser::Monadic(parser::Identity(_), ~parser::Niladic(parser::Array(_))) => {
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
