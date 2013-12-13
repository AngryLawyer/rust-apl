use eval::eval;
use eval::test_eval::test_eval;
use eval::eval::Printable;

#[test]
fn test_eval_basic_floor() {
    test_eval(~"⌊1", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"⌊0.2", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"⌊¯3.2", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, -4);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"⌊1J1", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, 1.0);
                assert_eq!(c.im, 1.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"⌊¯1J¯1.2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_eq!(c.re, -1.0);
                assert_eq!(c.im, -2.0);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}

#[test]
fn test_eval_array_floor() {
    test_eval(~"⌊1 0.1J2 1.1 1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplComplex(c)) => {
                        assert_eq!(c.im, 2.0)
                    },
                    _ => {
                        fail!(~"Bad array conjugation")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}
