use std::{result, option, int, str, float};

use parser;
use nodes;
use tokenizer;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_node};
use eval::array_helpers::simple_dyadic_array;

pub trait AddableValue {
    fn add(&self, other: &Value) -> result::Result<~Value, ~str>;
}

fn addFloat(f: &float, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok(~AplFloat(f + val))
        },
        &AplInteger(val) => {
            addFloat(f, ~AplFloat(val as float))
        },
        &AplComplex(ref _i, ref _j) => {
            addComplex(~AplComplex(~AplFloat(*f), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(addFloat, f, other)
        }
    }
}

fn addInteger(i: &int, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            addFloat(&(*i as float), other)
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(i + val))
        },
        &AplComplex(ref _i, ref _j) => {
            addComplex(~AplComplex(~AplInteger(*i), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(addInteger, i, other)
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
                &AplArray(_, _, _) => {
                    simple_dyadic_array(addComplex, complex, other)
                }
            }
        },
        _ => fail!(~"Oh dear")
    }
}

fn addArray(array: &Value, other: &Value) -> result::Result<~Value, ~str> {
    result::Err(~"Not yet implemented for arrays")
}

impl AddableValue for Value {

    fn add(&self, other: &Value) -> result::Result<~Value, ~str> {
        match self {
            &AplFloat(f) => {
                addFloat(&f, other)
            },
            &AplInteger(i) => {
                addInteger(&i, other)
            }
            &AplComplex(ref _i, ref _j) => {
                addComplex(self, other)
            },
            &AplArray(ref _depth, ref _dimensions, ref _values) => {
                addArray(self, other)
            }
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
