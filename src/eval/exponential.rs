use std::num::{sin, cos};
use extra::complex::Cmplx;
use nodes;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};
use math_constants::e;

pub fn exponential(first: &Value) -> Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            Ok(~AplFloat(val.exp()))
        },
        &AplInteger(val) => {
            Ok(~AplFloat((val as f64).exp()))
        },
        &AplComplex(c) => {
            let powed = e.pow(&c.re);
            let left = cos(c.im);
            let right = sin(c.im);
            let complex = Cmplx::new(left, right);
            let result = Cmplx::new(powed, 0.0) * complex;
            Ok(~AplComplex(result))
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(exponential, first)
        }
    }
}

pub fn eval_exponential(left: &nodes::Node) -> Result<~Value, ~str> {
    eval_monadic(exponential, left)
}
