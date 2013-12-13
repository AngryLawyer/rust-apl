use eval::eval;
use eval::test_eval::test_eval;
use eval::eval::Printable;

#[test]
fn test_eval_basic_sign() {
    test_eval(~"×5", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"×0", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"×¯5", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, -1);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"×1J1", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_approx_eq!(c.re, 0.707107);
                assert_approx_eq!(c.im, 0.707107);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}

#[test]
fn test_eval_array_sign() {
    test_eval(~"×0.5 ¯1 1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplInteger(-1)) => {
                    },
                    _ => {
                        fail!(~"Bad array sign")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}
