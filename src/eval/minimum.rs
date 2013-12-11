use std::result;

use nodes;
use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn minimum_float(f: f64, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            Ok(~AplFloat(if f < val { f } else { val }))
        },
        &AplInteger(val) => {
            Ok(if f < val as f64 { ~AplFloat(f) } else { ~AplInteger(val) })
        },
        &AplComplex(_c) => {
            Err(~"minimum is not supported on complex numbers")
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(minimum_float, f, other)
        }
    }
}

fn minimum_integer(i: int, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            Ok(if (i as f64) < val { ~AplInteger(i) } else { ~AplFloat(val) })
        },
        &AplInteger(val) => {
            Ok(~AplInteger(if i < val { i } else { val }))
        },
        &AplComplex(_c) => {
            Err(~"minimum is not supported on complex numbers")
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(minimum_integer, i, other)
        }
    }
}

fn minimum_array(array: &Value, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_) => {
            inverse_simple_dyadic_array(minimum, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(minimum, array, other)
        }
    }
}

pub fn minimum(first: &Value, other: &Value) -> Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            minimum_float(f, other)
        },
        &AplInteger(i) => {
            minimum_integer(i, other)
        }
        &AplComplex(_c) => {
            Err(~"minimum is not supported on complex numbers")
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            minimum_array(first, other)
        }
    }
}

pub fn eval_minimum(left: &nodes::Node, right: &nodes::Node) -> Result<~Value, ~str> {
    eval_dyadic(minimum, left, right)
}
