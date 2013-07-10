use std::result;
use eval;
use eval::Evaluator;

fn test_eval(input: ~str, f: &fn(result: ~eval::Value)) {

    let mut eval = Evaluator::new(input);
    match eval.eval() {
        result::Ok(result) => {
            f(result)
        },
        result::Err(msg) => {
            fail!(fmt!("%s - %s ", eval.parser.tokenizer.char_reader.source, msg))
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

    do test_eval(~"¯3") |result| {
        match result {
            ~eval::Integer(x) => {
                assert_eq!(x, -3);
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
    do test_eval(~"¯.2") |result| {
        match result {
            ~eval::Float(x) => {
                assert_eq!(x, -0.2f);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }
}

#[test]
fn test_eval_complex() {
    do test_eval(~"1J3") |result| {
        match result {
            ~eval::Complex(~eval::Integer(x), ~eval::Integer(y)) => {
                assert_eq!(x, 1);
                assert_eq!(y, 3);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    };

    do test_eval(~"¯1J.2") |result| {
        match result {
            ~eval::Complex(~eval::Integer(x), ~eval::Float(y)) => {
                assert_eq!(x, -1);
                assert!(y == 0.2f);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    };
}

#[test]
fn test_eval_addition() {
    do test_eval(~"1+1") |result| {
        match result {
            ~eval::Integer(x) => {
                assert_eq!(x, 2);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1.0+1") |result| {
        match result {
            ~eval::Float(x) => {
                assert_eq!(x, 2.0);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1+1J1") |result| {
        match result {
            ~eval::Complex(~eval::Integer(x), ~eval::Integer(y)) => {
                assert_eq!(x, 2);
                assert_eq!(y, 1);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1J2+3J4") |result| {
        match result {
            ~eval::Complex(~eval::Integer(x), ~eval::Integer(y)) => {
                assert_eq!(x, 4);
                assert_eq!(y, 6);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1J.2+3J4") |result| {
        match result {
            ~eval::Complex(~eval::Integer(x), ~eval::Float(y)) => {
                assert_eq!(x, 4);
                assert_eq!(y, 4.2);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1 1+1 1") |result| {
        match result {
            ~eval::Array(_order, array) => {
                match array[0] {
                    ~eval::Integer(2) => {
                        match array[1] {
                            ~eval::Integer(2) => {
                            },
                            _ => {
                                fail!(~"Bad array addition")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array addition")
                    }
                }
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"2+1 1") |result| {
        match result {
            ~eval::Array(_order, array) => {
                match array[0] {
                    ~eval::Integer(3) => {
                        match array[1] {
                            ~eval::Integer(3) => {
                            },
                            _ => {
                                fail!(~"Bad array addition")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array addition")
                    }
                }
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }
}

#[test]
fn test_eval_subtract() {
    do test_eval(~"1-1") |result| {
        match result {
            ~eval::Integer(x) => {
                assert_eq!(x, 0);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1.0-1") |result| {
        match result {
            ~eval::Float(x) => {
                assert_eq!(x, 0.0);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }
}
