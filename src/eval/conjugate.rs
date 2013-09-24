use nodes;
use std::result;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};
use eval::negate::negate;

pub fn conjugate(first: &Value) -> result::Result<~Value, ~str> {
    match first{
        &AplFloat(_) | &AplInteger(_) => {
            result::Ok(~(first.clone()))
        },
        &AplComplex(ref i, ref j) => {
            negate(*j).and_then(|new_j| {
                result::Ok(~AplComplex(i.clone(), new_j))
            })
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(conjugate, first)
        }
    }
}

pub fn eval_conjugate(left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_monadic(conjugate, left)
}
