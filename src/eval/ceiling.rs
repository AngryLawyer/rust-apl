use extra::complex::Cmplx;
use nodes;

use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn ceiling(first: &Value) -> Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            Ok(~AplInteger(val.ceil() as int))
        },
        &AplInteger(val) => {
            Ok(~AplInteger(val))
        },
        &AplComplex(c) => {
            Ok(~AplComplex(Cmplx::new(c.re.ceil() as f64, c.im.ceil() as f64)))
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(ceiling, first)
        }
    }
}

pub fn eval_ceiling(left: &nodes::Node) -> Result<~Value, ~str> {
    eval_monadic(ceiling, left)
}
