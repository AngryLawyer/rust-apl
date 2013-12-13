use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_multiplication() {
    test_eval(~"2×2", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 4);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2.0×2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 4.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2×1J1", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 2.0);
                assert_eq!(c.im, 2.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"4J5×3J2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 2.0);
                assert_eq!(c.im, 23.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"5J.2×3J.2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 14.96);
                assert_eq!(c.im, 1.6);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

}
    
#[test]
fn test_eval_array_multiplication() {
    test_eval(~"2×2 2", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(4), &~eval::AplInteger(4)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array multiplication: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2 2 × 2", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(4), &~eval::AplInteger(4)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array multiplication: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"3 3×2 1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(6), &~eval::AplInteger(3)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array multiplication: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    //TO- test length, depth
    test_eval_fail(~"1 1 1 × 1 1", |_result| {
        //Cool beanz
    });

}

