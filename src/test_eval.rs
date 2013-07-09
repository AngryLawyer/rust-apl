use std::result;
use eval;
use eval::Evaluator;
use eval::Value;

fn test_eval(input: ~str, f: &fn(result: ~eval::Value)) {

    let mut eval = Evaluator::new(input);
    match eval.eval() {
        result::Ok(result) => {
            f(result)
        },
        result::Err(msg) => {
            fail!(msg)
        }
    }
}

#[test]
fn test_eval_int() {
    do test_eval(~"3") |result| {
        match result {
            ~eval::Integer(x) => {
                assert_eq!(x, 3);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }
}

#[test]
fn test_eval_float() {
    do test_eval(~".2") |result| {
        match result {
            ~eval::Float(x) => {
                assert_eq!(x, 0.2f);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }
}
