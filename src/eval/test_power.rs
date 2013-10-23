use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_power() {
    do test_eval(~"2⋆3") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 8);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2.0⋆2") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 4.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2⋆1J1") |result| {
        match result {
            ~eval::AplComplex(~eval::AplFloat(x), ~eval::AplFloat(y)) => {
                assert_eq!(x, 1.53848);
                assert_eq!(y, 1.27792);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"3J4⋆1J2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplFloat(x), ~eval::AplFloat(y)) => {
                assert_eq!(x, -0.419813);
                assert_eq!(y, -0.660452);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }


}
    
#[test]
fn test_eval_array_power() {
    do test_eval(~"2⋆1 3") |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplInteger(8)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array power: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"2 0 ⋆ 1") |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplInteger(1)) => {
                        //Fine
                    },
                    _ => {
                        fail!(format!("Bad array power: got {}", result.to_string()))
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"3 3⋆2 4") |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(9) => {
                        match array[1] {
                            ~eval::AplInteger(81) => {
                            },
                            _ => {
                                fail!(~"Bad array power")
                            }
                        }
                    },
                    _ => {
                        fail!(~"Bad array power")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    }

    //TODO - test length, depth
    do test_eval_fail(~"1 1 1 ⋆ 1 1") |_result| {
        //Cool beanz
    }

}

