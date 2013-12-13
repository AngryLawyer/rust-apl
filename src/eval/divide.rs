use extra::complex::{Cmplx, Complex64};
use nodes;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};
use eval::add::add;
use eval::subtract::subtract;
use eval::multiply::multiply;

fn divide_float(f: f64, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(0.0) => {
            Err(~"Domain error - division by zero")
        },
        &AplFloat(val) => {
            Ok(~AplFloat(f / val))
        },
        &AplInteger(val) => {
            divide_float(f, &AplFloat(val as f64))
        },
        &AplComplex(_val) => {
            divide_complex(&Cmplx::new(f, 0.0), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(divide_float, f, other)
        }
    }
}

pub fn divide_integer(i: int, other:&Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_val) => {
            divide_float(i as f64, other)
        },
        &AplInteger(0) => {
            Err(~"Domain error - division by zero")
        },
        &AplInteger(val) => {
            let remainder = i % val;
            if remainder != 0 {
                divide_float(i as f64, &AplFloat(val as f64))
            } else {
                Ok(~AplInteger(i / val))
            }
        },
        &AplComplex(_val) => {
            divide_complex(&Cmplx::new(i as f64, 0.0), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(divide_integer, i, other)
        }
    }
}

fn divide_complex(c: &Complex64, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(f) => {
            divide_complex(c, &AplComplex(Cmplx::new(f, 0.0)))
        },
        &AplInteger(i) => {
            divide_complex(c, &AplComplex(Cmplx::new(i as f64, 0.0)))
        },
        &AplComplex(other_c) => {
            Ok(~AplComplex(c / other_c)) //FIXME: Doesn't catch divide by zero
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(divide_complex, c, other)
        }
    }
}

fn divide_array(array: &Value, other: &Value) -> Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_) => {
            inverse_simple_dyadic_array(divide, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(divide, array, other)
        }
    }
}

pub fn divide(first: &Value, other: &Value) -> Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            divide_float(f, other)
        },
        &AplInteger(i) => {
            divide_integer(i, other)
        }
        &AplComplex(ref c) => {
            divide_complex(c, other)
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            divide_array(first, other)
        }
    }
}

pub fn eval_division(left: &nodes::Node, right: &nodes::Node) -> Result<~Value, ~str> {
    eval_dyadic(divide, left, right)
}
