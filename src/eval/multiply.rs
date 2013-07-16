use nodes;
use std::result;
use eval::eval::{Value};


pub fn eval_multiplication(_left: &nodes::Node, _right: &nodes::Node) -> result::Result<~Value, ~str> {
    result::Err(~"Not yet implemented")
}
