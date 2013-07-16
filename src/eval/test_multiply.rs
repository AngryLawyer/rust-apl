use std::result;
use eval::eval;
use eval::eval::Evaluator;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_multiplication() {
    do test_eval(~"2×2") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 4);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2.0×2") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 4.0);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2×1J1") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(y)) => {
                assert_eq!(x, 2);
                assert_eq!(y, 2);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"3J4×1J2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(y)) => {
                assert_eq!(x, -5);
                assert_eq!(y, 10);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"5J.2×3J.2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplFloat(x), ~eval::AplFloat(y)) => {
                assert_eq!(x, 14.96);
                assert_eq!(y, 1.6);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

}
    
#[test]
fn test_eval_array_multiplication() {
    do test_eval(~"2×2 2") |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(4), &~eval::AplInteger(4)) => {
                        //Fine
                    },
                    _ => {
                        fail!(fmt!("Bad array multiplication: got %s", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2 2 × 2") |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(4), &~eval::AplInteger(4)) => {
                        //Fine
                    },
                    _ => {
                        fail!(fmt!("Bad array multiplication: got %s", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2J1×2 2") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplComplex(~eval::AplInteger(4), ~eval::AplInteger(2)) => {
                        match array[1] {
                            ~eval::AplComplex(~eval::AplInteger(4), ~eval::AplInteger(2)) => {
                            },
                            _ => {
                                fail!(~"Bad array multiplication")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array multiplication")
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"3 3×2 1") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(6) => {
                        match array[1] {
                            ~eval::AplInteger(3) => {
                            },
                            _ => {
                                fail!(~"Bad array multiplication")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array multiplication")
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    //TODO - test length, depth
    do test_eval_fail(~"1 1 1 × 1 1") |result| {
        //Cool beanz
    }

}

