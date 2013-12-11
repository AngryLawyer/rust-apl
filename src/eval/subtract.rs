use std::result;

use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn subtract_float(f: &f64, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok(~AplFloat(f - val))
        },
        &AplInteger(val) => {
            subtract_float(f, &AplFloat(val as f64))
        },
        &AplComplex(ref _i, ref _j) => {
            subtract_complex(&AplComplex(~AplFloat(*f), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(subtract_float, f, other)
        }
    }
}

fn subtract_integer(i: &int, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(_val) => {
            subtract_float(&(*i as f64), other)
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(i - val))
        },
        &AplComplex(ref _i, ref _j) => {
            subtract_complex(&AplComplex(~AplInteger(*i), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(subtract_integer, i, other)
        }
    }
}

fn subtract_complex(complex: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match complex {
        &AplComplex(ref i, ref j) => {
            match other {
                &AplFloat(_) | &AplInteger(_) => {
                    subtract_complex(complex, &AplComplex(~(other.clone()), ~AplInteger(0)))
                },
                &AplComplex(ref a, ref bi) => {
                    match (subtract(*i, *a), subtract(*j, *bi)) {
                        (result::Err(msg), _) => result::Err(msg),
                        (_, result::Err(msg)) => result::Err(msg),
                        (result::Ok(left), result::Ok(right)) => {
                            result::Ok(~AplComplex(left, right))
                        }
                    }
                },
                &AplArray(_, _, _) => {
                    simple_dyadic_array(subtract_complex, complex, other)
                }
            }
        },
        _ => fail!(~"Oh dear")
    }
}

fn subtract_array(array: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_, _) => {
            inverse_simple_dyadic_array(subtract, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(subtract, array, other)
        }
    }
}

pub fn subtract(first: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            subtract_float(&f, other)
        },
        &AplInteger(i) => {
            subtract_integer(&i, other)
        }
        &AplComplex(ref _i, ref _j) => {
            subtract_complex(first, other)
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            subtract_array(first, other)
        }
    }
}

pub fn eval_subtraction(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_dyadic(subtract, left, right)
}
