use nodes;

use std::result;
use eval::array_helpers::{simple_monadic_array};
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_monadic};
use eval::multiply::multiply;
use eval::add::add;

pub fn magnitude(first: &Value) -> result::Result<~Value, ~str> {
    match first {
        &AplFloat(val) => {
            result::Ok(~AplFloat(val.abs()))
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(val.abs()))
        },
        &AplComplex(ref i, ref j) => {
            multiply(*i, *i).chain( |ii| {
                multiply(*j, *j).chain( |jj| {
                    add(ii, jj).chain( |sum| {
                        match sum {
                            ~AplFloat(ref f) => {
                                result::Ok(~AplFloat(f.sqrt()))
                            },
                            ~AplInteger(ref integer) => {
                                result::Ok(~AplFloat((*integer as float).sqrt()))
                            },
                            _ => {
                                result::Err(~"Bad Magnitude")
                            }
                        }
                    })
                })
            })
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            simple_monadic_array(magnitude, first)
        }
    }
}

pub fn eval_magnitude(left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_monadic(magnitude, left)
}
