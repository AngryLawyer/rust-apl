use eval::eval;
use eval::test_eval::test_eval;
use eval::eval::Printable;

#[test]
fn test_eval_basic_magnitude() {
    do test_eval(~"|1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"|0.2") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 0.2);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"|¯1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"|3J4") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 5.0);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"|¯3J¯4") |result| {
        match result {
            ~eval::AplFloat(x) => {
                assert_eq!(x, 5.0);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }
}

#[test]
fn test_eval_array_magnitude() {
    do test_eval(~"|¯1 3J4") |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplFloat(5.0)) => {
                    },
                    _ => {
                        fail!(~"Bad array magnitude")
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }
}
