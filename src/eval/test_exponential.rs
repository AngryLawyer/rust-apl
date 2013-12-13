use eval::eval;
use eval::test_eval::test_eval;
use eval::eval::Printable;

#[test]
fn test_eval_basic_exponential() {
    test_eval(~"⋆1", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_approx_eq!(x, 2.71828183f64);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"⋆0.2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_approx_eq!(x, 1.22140276f64);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"⋆¯3.2", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_approx_eq!(x, 0.0407622);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"⋆1J1", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_approx_eq!(c.re, 1.46869);
                assert_approx_eq!(c.im, 2.28736);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"⋆¯1J¯1.2", |result| {
        match result {
            ~eval::AplComplex(c) => {
                assert_approx_eq!(c.re, 0.133304);
                assert_approx_eq!(c.im, -0.342878);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}

#[test]
fn test_eval_array_exponential() {
    test_eval(~"⋆1 0.1J2 1.1 1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplFloat(_), &~eval::AplComplex(c)) => {
                        assert_approx_eq!(c.im, 1.00493)
                    },
                    _ => {
                        fail!(~"Bad array exponential")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}
