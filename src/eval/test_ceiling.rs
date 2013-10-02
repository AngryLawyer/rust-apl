use eval::eval;
use eval::test_eval::test_eval;
use eval::eval::Printable;
#[test]
fn test_eval_basic_ceiling() {
    do test_eval(~"⌈1") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"⌈0.2") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, 1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"⌈¯3.2") |result| {
        match result {
            ~eval::AplInteger(x) => {
                assert_eq!(x, -3);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"⌈1J1") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(j)) => {
                assert_eq!(x, 1);
                assert_eq!(j, 1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }

    do test_eval(~"⌈¯1J¯1.2") |result| {
        match result {
            ~eval::AplComplex(~eval::AplInteger(x), ~eval::AplInteger(j)) => {
                assert_eq!(x, -1);
                assert_eq!(j, -1);
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }
}

#[test]
fn test_eval_array_ceiling() {
    do test_eval(~"⌈1 0.1J2 1.1 1") |result| {
        match result {
            ~eval::AplArray(ref _order, ref _dims, ref array) => {
                match (&array[0], &array[1]) {
                    (&~eval::AplInteger(1), &~eval::AplComplex(~eval::AplInteger(1), ~eval::AplInteger(j))) => {
                        assert_eq!(j, 2)
                    },
                    _ => {
                        fail!(~"Bad array conjugation")
                    }
                }
            },
            _ => {
                fail!(fmt!("Didn't find a number - %s", result.to_typed_string()));
            }
        }
    }
}
