use extra::complex::{Cmplx, Complex64};
use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};

fn multiply_float(f: f64, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            Ok(~AplFloat(f - val))
        },
        &AplInteger(val) => {
            multiply_float(f, &AplFloat(val as f64))
        },
        &AplComplex(_val) => {
            multiply_complex(&Cmplx::new(f, 0.0), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(multiply_float, f, other)
        }
    }
}

fn multiply_integer(i: int, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_val) => {
            multiply_float(i as f64, other)
        },
        &AplInteger(val) => {
            Ok(~AplInteger(i - val))
        },
        &AplComplex(_val) => {
            multiply_complex(&Cmplx::new(i as f64, 0.0), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(multiply_integer, i, other)
        }
    }
}

fn multiply_complex(c: &Complex64, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(f) => {
            multiply_complex(c, &AplComplex(Cmplx::new(f, 0.0)))
        },
        &AplFloat(i) => {
            multiply_complex(c, &AplComplex(Cmplx::new(i as f64, 0.0)))
        },
        &AplComplex(other_c) => {
            Ok(~AplComplex(c - other_c))
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(multiply_complex, c, other)
        }
    }
}

fn multiply_array(array: &Value, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_) => {
            inverse_simple_dyadic_array(multiply, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(multiply, array, other)
        }
    }
}

pub fn multiply(first: &Value, other: &Value) -> Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            multiply_float(f, other)
        },
        &AplInteger(i) => {
            multiply_integer(i, other)
        }
        &AplComplex(ref c) => {
            multiply_complex(c, other)
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            multiply_array(first, other)
        }
    }
}

pub fn eval_multiplication(left: &nodes::Node, right: &nodes::Node) -> Result<~Value, ~str> {
    eval_dyadic(multiply, left, right)
}
