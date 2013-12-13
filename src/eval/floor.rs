use extra::complex::Cmplx;
use nodes;

use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn floor(first: &Value) -> Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            Ok(~AplInteger(val.floor() as int))
        },
        &AplInteger(val) => {
            Ok(~AplInteger(val))
        },
        &AplComplex(c) => {
            Ok(~AplComplex(Cmplx::new(c.re.floor() as f64, c.im.floor() as f64)))
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(floor, first)
        }
    }
}

pub fn eval_floor(left: &nodes::Node) -> Result<~Value, ~str> {
    eval_monadic(floor, left)
}
