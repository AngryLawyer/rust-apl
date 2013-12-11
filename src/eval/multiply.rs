use nodes;
use std::result;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};
use eval::add::add;
use eval::subtract::subtract;

fn multiply_float(f: &f64, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(val) => {
            result::Ok(~AplFloat(f * val))
        },
        &AplInteger(val) => {
            multiply_float(f, &AplFloat(val as f64))
        },
        &AplComplex(ref _i, ref _j) => {
            multiply_complex(&AplComplex(~AplFloat(*f), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(multiply_float, f, other)
        }
    }
}

fn multiply_integer(i: &int, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(_val) => {
            multiply_float(&(*i as f64), other)
        },
        &AplInteger(val) => {
            result::Ok(~AplInteger(i * val))
        },
        &AplComplex(ref _i, ref _j) => {
            multiply_complex(&AplComplex(~AplInteger(*i), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(multiply_integer, i, other)
        }
    }
}

fn multiply_complex(complex: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match complex {
        &AplComplex(ref a, ref bi) => {
            match other {
                &AplFloat(_) | &AplInteger(_) => {
                    multiply_complex(complex, &AplComplex(~(other.clone()), ~AplInteger(0)))
                },
                &AplComplex(ref c, ref di) => {
                    //First, Outers, Inners, Lasts, negate lasts
                    match (multiply(*a, *c), multiply(*a, *di), multiply(*bi, *c), multiply(*bi, *di)) {
                        (result::Err(msg), _, _, _) |
                        (_, result::Err(msg), _, _) |
                        (_, _, result::Err(msg), _) |
                        (_, _, _, result::Err(msg)) => result::Err(msg),
                        (result::Ok(first), result::Ok(outer), result::Ok(inner), result::Ok(last)) => {
                            match (subtract(first, last), add(outer, inner)) {
                                (result::Err(msg), _) |
                                (_, result::Err(msg)) => result::Err(msg),
                                (result::Ok(real), result::Ok(imaginary)) => {
                                    result::Ok(~AplComplex(real, imaginary))
                                }
                            }
                        }
                    }
                },
                &AplArray(_, _, _) => {
                    simple_dyadic_array(multiply_complex, complex, other)
                }
            }
        },
        _ => fail!(~"Oh dear")
    }
}

fn multiply_array(array: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_, _) => {
            inverse_simple_dyadic_array(multiply, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(multiply, array, other)
        }
    }
}

pub fn multiply(first: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            multiply_float(&f, other)
        },
        &AplInteger(i) => {
            multiply_integer(&i, other)
        }
        &AplComplex(ref _i, ref _j) => {
            multiply_complex(first, other)
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            multiply_array(first, other)
        }
    }
}

pub fn eval_multiplication(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_dyadic(multiply, left, right)
}
