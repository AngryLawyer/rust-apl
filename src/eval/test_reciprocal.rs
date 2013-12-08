use eval::eval;
use eval::test_eval::test_eval;
use eval::eval::Printable;

#[test]
fn test_eval_basic_reciprocal() {
    test_eval(~"÷1", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"÷5", |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 0.2);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"÷¯1", |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, -1);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });

    test_eval(~"÷2J4", |result| {
        match result {
            ~eval::AplComplex(~eval::AplFloat(x), ~eval::AplFloat(j)) => {
                assert_eq!(x, 0.1);
                assert_eq!(j, -0.2);
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}

#[test]
fn test_eval_array_reciprocal() {
    test_eval(~"÷0.5 1J1", |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplFloat(2.0), complex) => {
                        assert_eq!(complex, &~eval::AplComplex(~eval::AplFloat(0.5), ~eval::AplFloat(-0.5)));
                    },
                    _ => {
                        fail!(~"Bad array reciprocal")
                    }
                }
            },
            _ => {
                fail!(format!("Didn't find a number - {}", result.to_typed_string()));
            }
        }
    });
}
