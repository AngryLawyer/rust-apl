use extra::complex::{Cmplx, Complex64};
use nodes;

use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn exponential(first: &Value) -> Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            Ok(~AplFloat(val.exp()))
        },
        &AplInteger(val) => {
            Ok(~AplFloat((val as f64).exp()))
        },
        &AplComplex(val) => {
            Ok(~AplComplex(Cmplx::new(val.re.exp(), val.im.exp())))
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(exponential, first)
        }
    }
}

pub fn eval_exponential(left: &nodes::Node) -> Result<~Value, ~str> {
    eval_monadic(exponential, left)
}
