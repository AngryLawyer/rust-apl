use std::{result, option, int, str, float};

use parser;
use nodes;
use tokenizer;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_node};

pub trait AddableValue {
    fn add(&self, other: &Value) -> result::Result<~Value, ~str>;
}

fn addFloat(f: float, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok(~AplFloat(f + val))
        },
        &AplInteger(val) => {
            addFloat(f, ~AplFloat(val as float))
        },
        &AplComplex(ref a, ref bi) => {
            addComplex(~AplComplex(~AplFloat(f), ~AplInteger(0)), other)
        },
        _ => {
            result::Err(~"Not yet implemented for arrays")
        }
    }
}

fn addInteger(i: int, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            addFloat(i as float, other)
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(i + val))
        },
        &AplComplex(ref a, ref bi) => {
            addComplex(~AplComplex(~AplInteger(i), ~AplInteger(0)), other)
        },
        _ => {
            result::Err(~"Not yet implemented for arrays")
        }
    }
}

fn addComplex(complex: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match complex {
        &AplComplex(ref i, ref j) => {
            match other {
                &AplFloat(_) | &AplInteger(_) => {
                    addComplex(complex, ~AplComplex(~(copy *other), ~AplInteger(0)))
                },
                &AplComplex(ref a, ref bi) => {
                    match (i.add(*a), j.add(*bi)) {
                        (result::Err(msg), _) => result::Err(msg),
                        (_, result::Err(msg)) => result::Err(msg),
                        (result::Ok(left), result::Ok(right)) => {
                            result::Ok(~AplComplex(left, right))
                        }
                    }
                },
                _ => {
                    result::Err(~"Not yet implemented for arrays")
                }
            }
        },
        _ => fail!(~"Oh dear")
    }
}

impl AddableValue for Value {

    fn add(&self, other: &Value) -> result::Result<~Value, ~str> {
        match self {
            &AplFloat(f) => {
                addFloat(f, other)
            },
            &AplInteger(i) => {
                addInteger(i, other)
            }
            &AplComplex(ref i, ref j) => {
                addComplex(self, other)
            },
            _ =>result::Err(~"Internal type mismatch")
        }
    }
}

pub fn eval_addition(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    match eval_node(left) {
        result::Ok(left) => {
            match eval_node(right) {
                result::Ok(right) => {
                    left.add(right)
                },
                result::Err(msg) => {
                    result::Err(msg)
                }
            }
        },
        result::Err(msg) => {
            result::Err(msg)
        }
    }
}
