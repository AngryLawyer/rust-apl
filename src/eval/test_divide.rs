use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_division() {
    test_eval(~"4÷2", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 2);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"5÷2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 2.5);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"-4÷20", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, -0.2);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"4.0÷2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 2.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"4÷2J2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 1.0);
                assert_eq!(c.im, -1.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"4J5÷3J2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_approx_eq!(c.re, 1.69230769);
                assert_approx_eq!(c.im, 0.538462);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"5J.2÷3J.2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_approx_eq!(c.re, 1.663717);
                assert_approx_eq!(c.im, -0.0442478);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

}
    
#[test]
fn test_eval_array_division() {
    test_eval(~"4÷2 2", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(2), &~eval::AplInteger(2)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array division: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"4 4 ÷ 2", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(2), &~eval::AplInteger(2)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array division: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"3 3÷2 1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplFloat(1.5), &~eval::AplInteger(3)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array division: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    //TO- test length, depth
    test_eval_fail(~"1 1 1 ÷ 1 1", |_result| {
        //Cool beanz
    });

}

