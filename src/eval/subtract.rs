use extra::complex::{Cmplx, Complex64};

use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn subtract_float(f: f64, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            Ok(~AplFloat(f - val))
        },
        &AplInteger(val) => {
            subtract_float(f, &AplFloat(val as f64))
        },
        &AplComplex(_val) => {
            subtract_complex(&Cmplx::new(f, 0.0), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(subtract_float, f, other)
        }
    }
}

fn subtract_integer(i: int, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_val) => {
            subtract_float(i as f64, other)
        },
        &AplInteger(val) => {
            Ok(~AplInteger(i - val))
        },
        &AplComplex(_val) => {
            subtract_complex(&Cmplx::new(i as f64, 0.0), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(subtract_integer, i, other)
        }
    }
}

fn subtract_complex(c: &Complex64, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(f) => {
            subtract_complex(c, &AplComplex(Cmplx::new(f, 0.0)))
        },
        &AplFloat(i) => {
            subtract_complex(c, &AplComplex(Cmplx::new(i as f64, 0.0)))
        },
        &AplComplex(other_c) => {
            Ok(~AplComplex(c - other_c))
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(subtract_complex, c, other)
        }
    }
}

fn subtract_array(array: &Value, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_) => {
            inverse_simple_dyadic_array(subtract, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(subtract, array, other)
        }
    }
}

pub fn subtract(first: &Value, other: &Value) -> Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            subtract_float(f, other)
        },
        &AplInteger(i) => {
            subtract_integer(i, other)
        }
        &AplComplex(ref c) => {
            subtract_complex(c, other)
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            subtract_array(first, other)
        }
    }
}

pub fn eval_subtraction(left: &nodes::Node, right: &nodes::Node) -> Result<~Value, ~str> {
    eval_dyadic(subtract, left, right)
}
