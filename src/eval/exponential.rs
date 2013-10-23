use nodes;

use std::result;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn exponential(first: &Value) -> result::Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            result::Ok(~AplInteger(val.ceil() as int))
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(val))
        },
        &AplComplex(ref i, ref j) => {
            do exponential(*i).and_then |ceil_i| {
                do exponential(*j).and_then |ceil_j| {
                    result::Ok(~AplComplex(ceil_i.clone(), ceil_j))
                }
            }
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(exponential, first)
        }
    }
}

pub fn eval_exponential(left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_monadic(exponential, left)
}
