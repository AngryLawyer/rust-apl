use extra::complex::{Cmplx, Complex64};

use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array};

fn add_float(f: f64, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            Ok(~AplFloat(f + val))
        },
        &AplInteger(val) => {
            add_float(f, &AplFloat(val as f64))
        },
        &AplComplex(_val) => {
            add_complex(&Cmplx::new(f, 0.0), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(add_float, f, other)
        }
    }
}

fn add_integer(i: int, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_val) => {
            add_float(i as f64, other)
        },
        &AplInteger(val) => {
            Ok(~AplInteger(i + val))
        },
        &AplComplex(_val) => {
            add_complex(&Cmplx::new(i as f64, 0.0), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(add_integer, i, other)
        }
    }
}

fn add_complex(c: &Complex64, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(f) => {
            add_complex(c, &AplComplex(Cmplx::new(f, 0.0)))
        },
        &AplInteger(i) => {
            add_complex(c, &AplComplex(Cmplx::new(i as f64, 0.0)))
        },
        &AplComplex(other_c) => {
            Ok(~AplComplex(c + other_c))
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(add_complex, c, other)
        }
    }
}

fn add_array(array: &Value, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            simple_dyadic_array(add_float, val, array)
        },
        &AplInteger(val) => {
            simple_dyadic_array(add_integer, val, array)
        },
        &AplComplex(val) => {
            simple_dyadic_array(add_complex, &val, array)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(add, array, other)
        }
    }
}

pub fn add(first: &Value, other: &Value) -> Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            add_float(f, other)
        },
        &AplInteger(i) => {
            add_integer(i, other)
        }
        &AplComplex(ref c) => {
            add_complex(c, other)
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            add_array(first, other)
        }
    }
}

pub fn eval_addition(left: &nodes::Node, right: &nodes::Node) -> Result<~Value, ~str> {
    eval_dyadic(add, left, right)
}
