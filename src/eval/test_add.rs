use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_addition() {
    do test_eval(~"1+1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 2);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"1.0+1") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 2.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
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
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
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
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
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
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }
}
    
#[test]
fn test_eval_array_addition() {

    do test_eval(~"2+1 1") |result| {
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
    } 

    do test_eval(~"1 1 + 2") |result| {
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
    }

    do test_eval(~"2J1+1 1") |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplComplex(~eval::AplInteger(3), ~eval::AplInteger(1)), &~eval::AplComplex(~eval::AplInteger(3), ~eval::AplInteger(1)))  => {
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
    }

    do test_eval(~"1 1+1 1") |result| {
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
    }

    //TODO - test length, depth
    do test_eval_fail(~"1 1 1 + 1 1") |_result| {
        //Cool beanz
    }

}
