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
fn test_eval_number() {
    do test_eval(~"3") |result| {
        match result {
            ~eval::Integer(3) => {
                //OK
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }
}
