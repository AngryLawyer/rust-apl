use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_addition() {
    test_eval(~"1+1", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 2);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"1.0+1", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 2.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"1+1J1", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 2.0f64);
                assert_eq!(c.im, 1.0f64);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"1J2+3J4", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 4.0f64);
                assert_eq!(c.im, 6.0f64);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"1J.2+3J4", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 4.0f64);
                assert_eq!(c.im, 4.2f64);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}
    
#[test]
fn test_eval_array_addition() {

    test_eval(~"2+1 1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(3), &~eval::AplInteger(3)) => {
                    },
                    _ => {
                        fail!(~"Bad array addition")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"1 1 + 2", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(3), &~eval::AplInteger(3)) => {
                    },
                    _ => {
                        fail!(~"Bad array addition")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2J1+1 1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplComplex(c1), &~eval::AplComplex(c2))  => {
                        assert_eq!(c1.re, 3.0);
                        assert_eq!(c1.im, 1.0);
                        assert_eq!(c2.re, 3.0);
                        assert_eq!(c2.im, 1.0);
                    },
                    _ => {
                        fail!(~"Bad array addition")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"1 1+1 1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(2), &~eval::AplInteger(2)) => {
                    },
                    _ => {
                        fail!(~"Bad array addition")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    //TODO - test length, depth
    test_eval_fail(~"1 1 1 + 1 1", |_result| {
        //Cool beanz
    })

}
