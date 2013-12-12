use extra::complex::Cmplx;
use nodes;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};

pub fn conjugate(first: &Value) -> Result<~Value, ~str> {
    match first{
        &AplFloat(_) | &AplInteger(_) => {
            Ok(~(first.clone()))
        },
        &AplComplex(c) => {
            Ok(~AplComplex(c.conj()))
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(conjugate, first)
        }
    }
}

pub fn eval_conjugate(left: &nodes::Node) -> Result<~Value, ~str> {
    eval_monadic(conjugate, left)
}
