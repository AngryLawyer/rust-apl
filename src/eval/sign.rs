use nodes;

use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};
use eval::divide::divide;
use eval::magnitude::magnitude;

pub fn sign(first: &Value) -> Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            Ok(if val < 0.0 {
                ~AplInteger(-1)
            } else if val > 0.0 {
                ~AplInteger(1)
            } else {
                ~AplInteger(0)
            })
        },
        &AplInteger(val) => {
            Ok(if val < 0 {
                ~AplInteger(-1)
            } else if val > 0 {
                ~AplInteger(1)
            } else {
                ~AplInteger(0)
            })
        },
        &AplComplex(_c) => {
            magnitude(first).and_then(|magnituded| {
                divide(first, magnituded)
            })
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(sign, first)
        }
    }
}

pub fn eval_sign(left: &nodes::Node) -> Result<~Value, ~str> {
    eval_monadic(sign, left)
}
