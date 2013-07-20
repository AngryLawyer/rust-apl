use eval::eval;
use eval::test_eval::test_eval;
use eval::eval::Printable;

#[test]
fn test_eval_basic_sign() {
    do test_eval(~"×5") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"×0") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 0);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"×¯5") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, -1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"×1J1") |result| {
        match result {
            ~eval::AplComplex(~eval::AplFloat(x), ~eval::AplFloat(j)) => {
                assert!(x.approx_eq(&0.707107));
                assert!(j.approx_eq(&0.707107));
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }
}

#[test]
fn test_eval_array_sign() {
    do test_eval(~"÷0.5 ¯1 1") |result| {
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
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }
}
