use std::num;

use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn power_float(f: &f64, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            if *f == 0.0 && val < 0.0 {
                Err(~"Cannot take 0 to a negative power")
            } else {
                Ok(~AplFloat(num::pow(*f, val)))
            }
        },
        &AplInteger(val) => {
            if *f == 0.0 && val < 0 {
                Err(~"Cannot take 0 to a negative power")
            } else {
                Ok(~AplFloat(num::pow(*f, val as f64)))
            }
        },
        &AplComplex(ref _i, ref _j) => {
            Err(~"power is not supported on complex numbers")
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(power_float, f, other)
        }
    }
}

fn power_integer(i: &int, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            if *i == 0 && val < 0.0 {
                Err(~"Cannot take 0 to a negative power")
            } else {
                Ok(~AplFloat(num::pow((*i as f64), val)))
            }
        },
        &AplInteger(val) => {
            if *i == 0 && val < 0 {
                Err(~"Cannot take 0 to a negative power")
            } else {
                Ok(~AplInteger(num::pow_with_uint(*i as uint, val as uint)))
            }
        },
        &AplComplex(ref _i, ref _j) => {
            Err(~"power is not supported on complex numbers")
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(power_integer, i, other)
        }
    }
}

fn power_array(array: &Value, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_, _) => {
            inverse_simple_dyadic_array(power, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(power, array, other)
        }
    }
}

pub fn power(first: &Value, other: &Value) -> Result<~Value, ~str> {
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

pub fn eval_power(left: &nodes::Node, right: &nodes::Node) -> Result<~Value, ~str> {
    eval_dyadic(power, left, right)
}

