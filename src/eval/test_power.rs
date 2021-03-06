use eval::eval;
use eval::test_eval::{test_eval, test_eval_fail};
use eval::eval::Printable;

#[test]
fn test_eval_basic_power() {
    test_eval(~"2⋆3", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 8);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2.0⋆2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 4.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2.0⋆1.2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_approx_eq!(x, 2.29739671);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"2⋆1J1", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_approx_eq!(c.re, 1.5384778);
                assert_approx_eq!(c.im, 1.27792255);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"3J4⋆2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, -7.0);
                assert_eq!(c.im, 24.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });


    test_eval_fail(~"0⋆¯1", |_result| {
        //No negative powers for zero
    });

    test_eval(~"¯27⋆1.2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_approx_eq!(c.re, -42.2274);
                assert_approx_eq!(c.im, -30.68);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"3J4⋆1J2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_approx_eq!(c.re, -0.419813);
                assert_approx_eq!(c.im, -0.660452);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });


}
    
#[test]
fn test_eval_array_power() {
    test_eval(~"2⋆1 3", |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(2), &~eval::AplInteger(8)) => {
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
    });

    test_eval(~"2 0 ⋆ 1", |result| {
        match result {
            ~eval::AplArray(_order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(2), &~eval::AplInteger(0)) => {
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
    });

    test_eval(~"3 3⋆2 0", |result| {
        match result {
            ~eval::AplArray(_order, _dims, array) => {
                match array[0] {
                    ~eval::AplInteger(9) => {
                        match array[1] {
                            ~eval::AplInteger(1) => {
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
    });

    //TO- test length, depth
    test_eval_fail(~"1 1 1 ⋆ 1 1", |_result| {
        //Cool beanz
    });
}

