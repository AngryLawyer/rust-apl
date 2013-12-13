use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_subtraction() {
    test_eval(~"1−1", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2.0−1", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 1.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"1−1J1", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 0.0);
                assert_eq!(c.im, -1.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"3J4−1J2", |result| {
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

    test_eval(~"5J.2−3J.2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 2.0);
                assert_eq!(c.im, 0.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

}
    
#[test]
fn test_eval_array_subtraction() {
    test_eval(~"2−1 1", |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplInteger(1)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array subtraction: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2 2 − 1", |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplInteger(1)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array subtraction: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2J1−1 1", |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplComplex(c) => {
                        assert_eq!(c.re, 1.0);
                        assert_eq!(c.im, 1.0);
                        match array[1] {
                            ~eval::AplComplex(c) => {
                                assert_eq!(c.re, 1.0);
                                assert_eq!(c.im, 1.0);
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
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"3 3−2 1", |result| {
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
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    //TO- test length, depth
    test_eval_fail(~"1 1 1 − 1 1", |_result| {
        //Cool beanz
    });

}
