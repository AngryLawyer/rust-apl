use std::{result, option, int, str, float};

use parser;
use nodes;
use tokenizer;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_node};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array};

fn add_float(f: &float, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok(~AplFloat(f + val))
        },
        &AplInteger(val) => {
            add_float(f, ~AplFloat(val as float))
        },
        &AplComplex(ref _i, ref _j) => {
            add_complex(~AplComplex(~AplFloat(*f), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(add_float, f, other)
        }
    }
}

fn add_integer(i: &int, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            add_float(&(*i as float), other)
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(i + val))
        },
        &AplComplex(ref _i, ref _j) => {
            add_complex(~AplComplex(~AplInteger(*i), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(add_integer, i, other)
        }
    }
}

fn add_complex(complex: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match complex {
        &AplComplex(ref i, ref j) => {
            match other {
                &AplFloat(_) | &AplInteger(_) => {
                    add_complex(complex, ~AplComplex(~(copy *other), ~AplInteger(0)))
                },
                &AplComplex(ref a, ref bi) => {
                    match (add(*i, *a), add(*j, *bi)) {
                        (result::Err(msg), _) => result::Err(msg),
                        (_, result::Err(msg)) => result::Err(msg),
                        (result::Ok(left), result::Ok(right)) => {
                            result::Ok(~AplComplex(left, right))
                        }
                    }
                },
                &AplArray(_, _, _) => {
                    simple_dyadic_array(add_complex, complex, other)
                }
            }
        },
        _ => fail!(~"Oh dear")
    }
}

fn add_array(array: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            simple_dyadic_array(add_float, &val, array)
        },
        &AplInteger(val) => {
            simple_dyadic_array(add_integer, &val, array)
        },
        &AplComplex(_, _) => {
            simple_dyadic_array(add_complex, other, array)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(add, array, other)
        }
    }
}

fn add(first: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            add_float(&f, other)
        },
        &AplInteger(i) => {
            add_integer(&i, other)
        }
        &AplComplex(ref _i, ref _j) => {
            add_complex(first, other)
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            add_array(first, other)
        }
    }
}

pub fn eval_addition(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    match eval_node(left) {
        result::Ok(left) => {
            match eval_node(right) {
                result::Ok(right) => {
                    add(left, right)
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
