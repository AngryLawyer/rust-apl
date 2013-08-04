use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;
use std::float::ApproxEq;

#[test]
fn test_eval_basic_division() {
    do test_eval(~"4÷2") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 2);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"5÷2") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 2.5);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"-4÷20") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, -0.2);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"4.0÷2") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 2.0);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"4÷2J2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(y)) => {
                assert_eq!(x, 1);
                assert_eq!(y, -1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"4J5÷3J2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplFloat(x), ~eval::AplFloat(y)) => {
                assert!(x.approx_eq(&1.69230769));
                assert!(y.approx_eq(&0.538462));
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"5J.2÷3J.2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplFloat(x), ~eval::AplFloat(y)) => {
                assert!(x.approx_eq(&1.663717));
                assert!(y.approx_eq(&-0.0442478));
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

}
    
#[test]
fn test_eval_array_division() {
    do test_eval(~"4÷2 2") |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(2), &~eval::AplInteger(2)) => {
                        //Fine
                    },
                    _ => {
                        fail!(fmt!("Bad array division: got %s", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"4 4 ÷ 2") |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(2), &~eval::AplInteger(2)) => {
                        //Fine
                    },
                    _ => {
                        fail!(fmt!("Bad array division: got %s", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2J1÷2 2") |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplComplex(~eval::AplInteger(1), ~eval::AplFloat(0.5)), &~eval::AplComplex(~eval::AplInteger(1), ~eval::AplFloat(0.5))) => {
                    },
                    _ => {
                        fail!(fmt!("Bad array division: got %s", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"3 3÷2 1") |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplFloat(1.5), &~eval::AplInteger(3)) => {
                        //Fine
                    },
                    _ => {
                        fail!(fmt!("Bad array division: got %s", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    //TODO - test length, depth
    do test_eval_fail(~"1 1 1 ÷ 1 1") |_result| {
        //Cool beanz
    }

}

