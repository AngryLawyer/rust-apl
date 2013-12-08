use std::result;
use eval::eval;
use eval::eval::Evaluator;

pub fn test_eval(input: ~str, f: |result: ~eval::Value|) {

    let mut eval = Evaluator::new(input);
    match eval.eval() {
        result::Ok(result) => {
            f(result)
        },
        result::Err(msg) => {
            fail!(format!("{} - {}", eval.parser.tokenizer.char_reader.source, msg))
        }
    }
}

pub fn test_eval_fail(input: ~str, f: |result: ~str|) {

    let mut eval = Evaluator::new(input);
    match eval.eval() {
        result::Ok(_) => {
            fail!(format!("{} - incorrectly gave a success", eval.parser.tokenizer.char_reader.source))
        },
        result::Err(msg) => {
            f(msg)
        }
    }
}

#[test]
fn test_eval_int() {
    test_eval(~"3", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 3);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    });

    test_eval(~"¯3", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, -3);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    })
}

#[test]
fn test_eval_float() {
    test_eval(~".2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 0.2f64);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    });
    test_eval(~"¯.2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, -0.2f64);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    })
}

#[test]
fn test_eval_complex() {
    test_eval(~"1J3", |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(y)) => {
                assert_eq!(x, 1);
                assert_eq!(y, 3);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    });

    test_eval(~"¯1J.2", |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplFloat(y)) => {
                assert_eq!(x, -1);
                assert!(y == 0.2f64);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    });
}
