use nodes;
use std::result;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_node, eval_dyadic};


pub fn eval_multiplication(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    result::Err(~"Not yet implemented")
}
