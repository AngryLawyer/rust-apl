use nodes;

use std::result;
use eval::eval::{Value, eval_monadic};
use eval::divide::divide_integer;

pub fn reciprocal(first: &Value) -> result::Result<~Value, ~str> {
    divide_integer(&1, first)
}

pub fn eval_reciprocal(left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_monadic(reciprocal, left)
}
