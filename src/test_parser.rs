use std::result;
use parser::Parser;
use nodes;


fn test_parse(input: ~str, f: |tree: ~nodes::Node|) {

    let mut parser = Parser::new(input);
    match parser.parse_next_statement() {
        result::Ok(tree) => {
            f(tree)
        },
        result::Err(msg) => {
            fail!(msg)
        }
    }
}

#[test]
fn test_parse_number() {
    
    test_parse(~"3.141", |tree| {
        match tree {
            ~nodes::Array(_) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    });

    test_parse(~"0J3.141", |tree| {
        match tree {
            ~nodes::Array(_) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    });
}

#[test]
fn test_parse_array() {

    test_parse(~"1 2 3 4", |tree| {
        match tree {
            ~nodes::Array(_) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find an array");
            }
        }
    });
}

#[test]
fn test_parse_variable() {
   
    test_parse(~"Trololo", |tree| {
        match tree {
            ~nodes::Variable(_) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find a variable");
            }
        }
    });
}

#[test]
fn test_parse_zilde() {
   
    test_parse(~"⍬", |tree| {
        match tree {
            ~nodes::Zilde(_) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find zilde");
            }
        }
    });
}

#[test]
fn test_conjugate() {

    test_parse(~"+1", |tree| {
        match tree {
            ~nodes::Conjugate(_, ~nodes::Array(_)) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find conjugate one");
            }
        }
    });

}

#[test]
fn test_negate() {
    test_parse(~"-1", |tree| {
        match tree {
            ~nodes::Negate(_, ~nodes::Array(_)) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find conjugate one");
            }
        }
    });
}

#[test]
fn test_addition() {
    test_parse(~"1 2 3 4 + 2 4 6 8", |tree| {
        match tree {
            ~nodes::Addition(_, ~nodes::Array(_), ~nodes::Array(_)) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find the right Dyadic expression");
            }
        }
    });
}
