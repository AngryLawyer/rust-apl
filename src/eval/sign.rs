use nodes;

use std::result;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn sign(first: &Value) -> result::Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            result::Ok(if val < 0.0 {
                ~AplInteger(-1)
            } else if val > 0.0 {
                ~AplInteger(1)
            } else {
                ~AplInteger(0)
            })
        },
        &AplInteger(val) => {
            result::Ok(if val < 0 {
                ~AplInteger(-1)
            } else if val > 0 {
                ~AplInteger(1)
            } else {
                ~AplInteger(0)
            })
        },
        &AplComplex(ref _i, ref _j) => {
            result::Err(~"Sign for Complex not yet implemented")
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(sign, first)
        }
    }
}

pub fn eval_sign(left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_monadic(sign, left)
}
