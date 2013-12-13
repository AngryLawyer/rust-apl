use nodes;

use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn magnitude(first: &Value) -> Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            Ok(~AplFloat(val.abs()))
        },
        &AplInteger(val) => {
            Ok(~AplInteger(val.abs()))
        },
        &AplComplex(c) => {
            let ii = c.re * c.re;
            let jj = c.im * c.im;
            let iijj = ii + jj;
            Ok(~AplFloat(iijj.sqrt()))
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(magnitude, first)
        }
    }
}

pub fn eval_magnitude(left: &nodes::Node) -> Result<~Value, ~str> {
    eval_monadic(magnitude, left)
}
