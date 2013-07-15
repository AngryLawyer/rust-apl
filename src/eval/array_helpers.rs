use eval::eval::{AplArray, Value};
use std::{result, uint};

pub fn simple_dyadic_array<T>(func: extern fn(&T, &Value) -> result::Result<~Value, ~str>, param: &T, other: &Value) -> result::Result<~Value, ~str> {
    match other {
        &AplArray(ref depth, ref dimensions, ref values) => {
            let mut result_values: ~[~Value] = ~[];
            let mut error_state = ~"";
            let mut errored = false;

            for values.iter().advance |value|{ 
                if !errored {
                    match func(param, *value) {
                        result::Ok(val) => {
                            result_values.push(val);
                        },
                        result::Err(err) => {
                            errored = true;
                            error_state = err;
                        }
                    }
                }
            }

            if errored {
                result::Err(error_state)
            } else {
                result::Ok(~AplArray(copy *depth, copy *dimensions, result_values))
            }
        },
        _ => {
            fail!("This should never be reached")
        }
    }
}

pub fn inverse_simple_dyadic_array<T>(func: extern fn(&Value, &Value) -> result::Result<~Value, ~str>, param: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match param {
        &AplArray(ref depth, ref dimensions, ref values) => {
            let mut result_values: ~[~Value] = ~[];
            let mut error_state = ~"";
            let mut errored = false;

            for values.iter().advance |value|{ 
                if !errored {
                    match func(*value, param) {
                        result::Ok(val) => {
                            result_values.push(val);
                        },
                        result::Err(err) => {
                            errored = true;
                            error_state = err;
                        }
                    }
                }
            }

            if errored {
                result::Err(error_state)
            } else {
                result::Ok(~AplArray(copy *depth, copy *dimensions, result_values))
            }
        },
        _ => {
            fail!("This should never be reached")
        }
    }
}

pub fn dual_dyadic_array(func: extern fn(&Value, &Value) -> result::Result<~Value, ~str>, param: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match param {
        &AplArray(ref left_depth, ref left_dimensions, ref left_values) => {
            match other {
                &AplArray(ref right_depth, ref right_dimensions, ref right_values) => {
                    //Different depths are considered a rank error
                    //Different shapes are considered a length error
                    if left_depth != right_depth {
                        return result::Err(~"Rank error")
                    } else if left_dimensions != right_dimensions {
                        return result::Err(~"Length error")
                    }

                    let mut result_values: ~[~Value] = ~[];
                    let mut error_state = ~"";
                    let mut errored = false;

                    for uint::range(0, left_values.len()) |index| {
                        if !errored {
                            match func(left_values[index], right_values[index]) {
                                result::Ok(val) => {
                                    result_values.push(val);
                                },
                                result::Err(err) => {
                                    errored = true;
                                    error_state = err;
                                }
                            }
                        }
                    }

                    if errored {
                        result::Err(error_state)
                    } else {
                        result::Ok(~AplArray(copy *left_depth, copy *left_dimensions, result_values))
                    }
                },
                _ => {
                    fail!("This should never be reached")
                }
            }
        },
        _ => {
            fail!("This should never be reached")
        }
    }
}
