use nodes;

use std::result;
use eval::eval::{AplComplex, Value, eval_monadic};
use eval::divide::{divide_integer, divide};
use eval::multiply::multiply;
use eval::conjugate::conjugate;
use eval::add::add;

pub fn reciprocal(first: &Value) -> result::Result<~Value, ~str> {
    match first {
        &AplComplex(ref a, ref bi) => {
            conjugate(first).chain(|conjugated| {
                match conjugated {
                    ~AplComplex(c, di) => {
                        multiply(*a, *a).chain(|aa| {
                            multiply(*bi, *bi).chain(|bibi| {
                                add(aa, bibi).chain(|div| {
                                    divide(c, div).chain(|real| {
                                        divide(di, div).chain(|imaginary| {
                                            result::Ok(~AplComplex(real.clone(), imaginary))
                                        })
                                    })
                                })
                            })
                        })
                    },
                    _ => fail!(~"Conjugation error")
                }
            })
        },
        _ => {
            print("LOL");
            divide_integer(&1, first)
        }
    }
}

pub fn eval_reciprocal(left: &nodes::Node) -> result::Result<~Value, ~str> {
    eval_monadic(reciprocal, left)
}
