use std::result;
use eval::eval;
use eval::eval::Evaluator;
use eval::test_eval::{test_eval, test_eval_fail};

#[test]
fn test_eval_basic_subtraction() {
    do test_eval(~"1−1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 0);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"2.0−1") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 1.0);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1−1J1") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(y)) => {
                assert_eq!(x, 0);
                assert_eq!(y, -1);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"3J4−1J2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(y)) => {
                assert_eq!(x, 2);
                assert_eq!(y, 2);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"5J.2−3J.2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplFloat(y)) => {
                assert_eq!(x, 2);
                assert_eq!(y, 0.0);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

}
    
#[test]
fn test_eval_array_subtraction() {
    do test_eval(~"2−1 1") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(1) => {
                        match array[1] {
                            ~eval::AplInteger(1) => {
                            },
                            _ => {
                                fail!(~"Bad array subtraction")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array subtraction")
                    }
                }
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"2 2 − 1") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(1) => {
                        match array[1] {
                            ~eval::AplInteger(1) => {
                            },
                            _ => {
                                fail!(~"Bad array subtraction")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array subtraction")
                    }
                }
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"2J1−1 1") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplComplex(~eval::AplInteger(1), ~eval::AplInteger(1)) => {
                        match array[1] {
                            ~eval::AplComplex(~eval::AplInteger(1), ~eval::AplInteger(1)) => {
                            },
                            _ => {
                                fail!(~"Bad array subtraction")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array subtraction")
                    }
                }
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"3 3−2 1") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(1) => {
                        match array[1] {
                            ~eval::AplInteger(2) => {
                            },
                            _ => {
                                fail!(~"Bad array subtraction")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array subtraction")
                    }
                }
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    //TODO - test length, depth
    do test_eval_fail(~"1 1 1 − 1 1") |result| {
        //Cool beanz
    }

}
