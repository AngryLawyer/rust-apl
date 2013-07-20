use eval::eval;
use eval::test_eval::test_eval;
use eval::eval::Printable;

#[test]
fn test_eval_basic_reciprocal() {
    do test_eval(~"÷1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"÷5") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 0.2);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"÷¯1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, -1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"÷1J1") |result| {
        match result {
            ~eval::AplComplex(~eval::AplFloat(x), ~eval::AplFloat(j)) => {
                assert_eq!(x, 0.5);
                assert_eq!(j, -0.5);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }
}

#[test]
fn test_eval_array_reciprocal() {
    do test_eval(~"÷0.5 1J1 1 1") |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(2), &~eval::AplComplex(~eval::AplFloat(0.5), ~eval::AplFloat(j))) => {
                        assert_eq!(j, -0.5)
                    },
                    _ => {
                        fail!(~"Bad array reciprocal")
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }
}
