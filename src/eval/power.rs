use std::result;

use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn power_float(f: &f64, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok(~AplFloat(if *f > val { *f } else { val }))
        },
        &AplInteger(val) => {
            result::Ok((if *f > val as f64 { ~AplFloat(*f) } else { ~AplInteger(val) }))
        },
        &AplComplex(ref _i, ref _j) => {
            Err(~"power is not supported on complex numbers")
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(power_float, f, other)
        }
    }
}

fn power_integer(i: &int, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok((if *i as f64 > val { ~AplInteger(*i) } else { ~AplFloat(val) }))
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(if *i > val { *i } else { val }))
        },
        &AplComplex(ref _i, ref _j) => {
            Err(~"power is not supported on complex numbers")
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(power_integer, i, other)
        }
    }
}

fn power_array(array: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_, _) => {
            inverse_simple_dyadic_array(power, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(power, array, other)
        }
    }
}

pub fn power(first: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            power_float(&f, other)
        },
        &AplInteger(i) => {
            power_integer(&i, other)
        }
        &AplComplex(ref _i, ref _j) => {
            Err(~"power is not supported on complex numbers")
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            power_array(first, other)
        }
    }
}

pub fn eval_power(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_dyadic(power, left, right)
}

