use nodes;
use std::result;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn negate(first: &Value) -> result::Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            result::Ok(~AplFloat(-f))
        },
        &AplInteger(i) => {
            result::Ok(~AplInteger(-i))
        }
        &AplComplex(ref i, ref j) => {
            negate(*i).chain(|new_i| {
                negate(*j).chain(|new_j| {
                    result::Ok(~AplComplex(copy new_i, copy new_j))
                })
            })
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(negate, first)
        }
    }
}

pub fn eval_negate(left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_monadic(negate, left)
}
