use std::result;
use eval::eval;
use eval::eval::Evaluator;
use eval::test_eval::test_eval;

#[test]
fn test_eval_basic_addition() {
    do test_eval(~"1+1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 2);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1.0+1") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 2.0);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"1+1J1") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(y)) => {
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
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(y)) => {
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
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplFloat(y)) => {
                assert_eq!(x, 4);
                assert_eq!(y, 4.2);
            },
            _ => {
                fail!(~"Didn't find a number");
            }
        }
    }

    do test_eval(~"2+1 1") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(3) => {
                        match array[1] {
                            ~eval::AplInteger(3) => {
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
fn test_eval_array_addition() {
    do test_eval(~"1 1 + 2") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(3) => {
                        match array[1] {
                            ~eval::AplInteger(3) => {
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

    do test_eval(~"2J1+1 1") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplComplex(~eval::AplInteger(3), ~eval::AplInteger(1)) => {
                        match array[1] {
                            ~eval::AplComplex(~eval::AplInteger(3), ~eval::AplInteger(1)) => {
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

    do test_eval(~"1 1+1 1") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(2) => {
                        match array[1] {
                            ~eval::AplInteger(2) => {
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

    //TODO - test length, depth

}
