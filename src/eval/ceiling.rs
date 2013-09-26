use nodes;

use std::result;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn ceiling(first: &Value) -> result::Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            result::Ok(~AplInteger(val.ceil() as int))
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(val))
        },
        &AplComplex(ref i, ref j) => {
            do ceiling(*i).and_then |ceil_i| {
                do ceiling(*j).and_then |ceil_j| {
                    result::Ok(~AplComplex(ceil_i.clone(), ceil_j))
                }
            }
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(ceiling, first)
        }
    }
}

pub fn eval_ceiling(left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_monadic(ceiling, left)
}
