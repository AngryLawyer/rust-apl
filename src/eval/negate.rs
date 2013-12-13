use nodes;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn negate(first: &Value) -> Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            Ok(~AplFloat(-f))
        },
        &AplInteger(i) => {
            Ok(~AplInteger(-i))
        }
        &AplComplex(c) => {
            Ok(~AplComplex(-c))
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(negate, first)
        }
    }
}

pub fn eval_negate(left: &nodes::Node) -> Result<~Value, ~str> {
    eval_monadic(negate, left)
}
