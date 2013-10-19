use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_minimum() {
    do test_eval(~"1⌊1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2.0⌊1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval_fail(~"1⌊1J1") |_result| {
        //Cool beanz
    }

    do test_eval_fail(~"3J4⌊1J2") |_result| {
        //Cool beanz
    }


}
    
#[test]
fn test_eval_array_minimum() {
    do test_eval(~"2⌊1 3") |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplInteger(2)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array minimum: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2 0 ⌊ 1") |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplInteger(0)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array minimum: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval_fail(~"2J1⌊1 1") |_result| {
        //Cool beanz
    }

    do test_eval(~"3 3⌊2 4") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(2) => {
                        match array[1] {
                            ~eval::AplInteger(3) => {
                            },
                            _ => {
                                fail!(~"Bad array minimum")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array minimum")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    //TODO - test length, depth
    do test_eval_fail(~"1 1 1 ⌊ 1 1") |_result| {
        //Cool beanz
    }

}


