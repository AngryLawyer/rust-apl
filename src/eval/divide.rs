use nodes;
use std::result;
use eval::eval::{AplFloat, AplInteger, AplComplex, AplArray, Value, eval_dyadic};
use eval::array_helpers::{simple_dyadic_array, dual_dyadic_array, inverse_simple_dyadic_array};
use eval::add::add;
use eval::subtract::subtract;
use eval::multiply::multiply;

fn divide_float(f: &float, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(0.0) => {
            result::Err(~"Domain error - division by zero")
        },
        &AplFloat(val) => {
            result::Ok(~AplFloat(f / val))
        },
        &AplInteger(val) => {
            divide_float(f, ~AplFloat(val as float))
        },
        &AplComplex(ref _i, ref _j) => {
            divide_complex(~AplComplex(~AplFloat(*f), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(divide_float, f, other)
        }
    }
}

pub fn divide_integer(i: &int, other:&Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(_val) => {
            divide_float(&(*i as float), other)
        },
        &AplInteger(0) => {
            result::Err(~"Domain error - division by zero")
        },
        &AplInteger(val) => {
            let remainder = i % val;
            if remainder != 0 {
                divide_float(&(*i as float), ~AplFloat(val as float))
            } else {
                result::Ok(~AplInteger(i / val))
            }
        },
        &AplComplex(ref _i, ref _j) => {
            divide_complex(~AplComplex(~AplInteger(*i), ~AplInteger(0)), other)
        },
        &AplArray(_, _, _) => {
            simple_dyadic_array(divide_integer, i, other)
        }
    }
}

fn divide_complex(complex: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match complex {
        &AplComplex(ref a, ref bi) => {
            match other {
                &AplFloat(_) | &AplInteger(_) => {
                    divide_complex(complex, ~AplComplex(~(other.clone()), ~AplInteger(0)))
                },
                &AplComplex(ref c, ref di) => {
                    let za = multiply(*a, *c).and_then(|ac| {
                        multiply(*bi, *di).and_then(|bidi| {
                            multiply(*c, *c).and_then(|cc| {
                                multiply(*di, *di).and_then(|didi| {
                                    add(ac, bidi).and_then(|left| {
                                        add(cc, didi).and_then(|right| {
                                            divide(left, right)
                                        })
                                    })
                                })
                            })
                        })
                    });
                    let zb = multiply(*bi, *c).and_then(|bic| {
                        multiply(*a, *di).and_then(|adi| {
                            multiply(*c, *c).and_then(|cc| {
                                multiply(*di, *di).and_then(|didi| {
                                    subtract(bic, adi).and_then(|left| {
                                        add(cc, didi).and_then(|right| {
                                            divide(left, right)
                                        })
                                    })
                                })
                            })
                        })
                    });
                    match (za, zb) {
                        (result::Err(err), _) | (_, result::Err(err)) => {
                            result::Err(err)
                        },
                        (result::Ok(left), result::Ok(right))=> {
                            result::Ok(~AplComplex(left, right))
                        } 
                    }
                    //z.a = (x.a*y.a + x.b*y.b)/(y.a*y.a+y.b*y.b);
                    //z.b = (x.b*y.a - x.a*y.b)/(y.a*y.a + y.b*y.b);
                },
                &AplArray(_, _, _) => {
                    simple_dyadic_array(divide_complex, complex, other)
                }
            }
        },
        _ => fail!(~"Oh dear")
    }
}

fn divide_array(array: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match other {
        &AplFloat(_) |  &AplInteger(_) | &AplComplex(_, _) => {
            inverse_simple_dyadic_array(divide, array, other)
        },
        &AplArray(_, _, _) => {
            dual_dyadic_array(divide, array, other)
        }
    }
}

pub fn divide(first: &Value, other: &Value) -> result::Result<~Value, ~str> {
    match first{
        &AplFloat(f) => {
            divide_float(&f, other)
        },
        &AplInteger(i) => {
            divide_integer(&i, other)
        }
        &AplComplex(ref _i, ref _j) => {
            divide_complex(first, other)
        },
        &AplArray(ref _depth, ref _dimensions, ref _values) => {
            divide_array(first, other)
        }
    }
}

pub fn eval_division(left: &nodes::Node, right: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_dyadic(divide, left, right)
}
