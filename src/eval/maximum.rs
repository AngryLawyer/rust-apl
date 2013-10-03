use std::result;

use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn maximum_float(f: &f64, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok(~AplFloat(if *f > val { *f } else { val }))
        },
        &AplInteger(val) => {
            result::Ok((if *f > val as f64 { ~AplFloat(*f) } else { ~AplInteger(val) }))
        },
        &AplComplex(ref _i, ref _j) => {
            Err(~"Maximum is not supported on complex numbers")
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(maximum_float, f, other)
        }
    }
}

fn maximum_integer(i: &int, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok((if *i as f64 > val { ~AplInteger(*i) } else { ~AplFloat(val) }))
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(if *i > val { *i } else { val }))
        },
        &AplComplex(ref _i, ref _j) => {
            Err(~"Maximum is not supported on complex numbers")
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(maximum_integer, i, other)
        }
    }
}

fn maximum_array(array: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_, _) => {
            inverse_simple_dyadic_array(maximum, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(maximum, array, other)
        }
    }
}

pub fn maximum(first: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            maximum_float(&f, other)
        },
        &AplInteger(i) => {
            maximum_integer(&i, other)
        }
        &AplComplex(ref _i, ref _j) => {
            Err(~"Maximum is not supported on complex numbers")
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            maximum_array(first, other)
        }
    }
}

pub fn eval_maximum(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_dyadic(maximum, left, right)
}

