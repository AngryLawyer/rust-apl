use tokenizer;
use tokenizer::Token;
use tokenizer::TokenData;
use eval::eval::Value;
use parser::Parser;

use eval::add::eval_addition;
use eval::subtract::eval_subtraction;
use eval::multiply::eval_multiplication;
use eval::divide::eval_division;
use eval::maximum::eval_maximum;
use eval::minimum::eval_minimum;
use eval::exponential::eval_exponential;

use eval::conjugate::eval_conjugate;
use eval::negate::eval_negate;
use eval::reciprocal::eval_reciprocal;
use eval::sign::eval_sign;
use eval::magnitude::eval_magnitude;
use eval::ceiling::eval_ceiling;
use eval::floor::eval_floor;
use eval::power::eval_power;

pub trait EvalNode {
    fn eval(&self) -> Result<~Value, ~str>;
}

pub trait Parseable {
    fn monadic(&self, parser: &mut Parser) -> Result<~Node, ~str>;
    fn dyadic(&self, parser: &mut Parser, left: ~Node) -> Result<~Node, ~str>;
}

impl Parseable for TokenData {
    fn monadic(&self, parser: &mut Parser) -> Result<~Node, ~str> {
        match self.string {
            ~"+" => parser.create_monadic_result(Conjugate),
            ~"-" | ~"−" => parser.create_monadic_result(Negate),
            ~"×" => parser.create_monadic_result(Sign),
            ~"÷" => parser.create_monadic_result(Reciprocal),
            ~"|" | ~"∣" => parser.create_monadic_result(Magnitude),
            ~"⌈" => parser.create_monadic_result(Ceiling),
            ~"⌊" => parser.create_monadic_result(Floor),
            ~"⋆" | ~"*" => parser.create_monadic_result(Exponential),
            _ => parser.parse_base_expression()
        }
    }

    fn dyadic(&self, parser: &mut Parser, left: ~Node) -> Result<~Node, ~str> {
        match self.string {
            ~"+" => parser.create_dyadic_result(left, Addition),
            ~"-" | ~"−" => parser.create_dyadic_result(left, Subtraction),
            ~"×" => parser.create_dyadic_result(left, Multiplication),
            ~"÷" => parser.create_dyadic_result(left, Division),
            ~"⌈" => parser.create_dyadic_result(left, Maximum),
            ~"⌊" => parser.create_dyadic_result(left, Minimum),
            ~"⋆" | ~"*" => parser.create_dyadic_result(left, Power),
            _ => Err(~"Unknown operator")
        }
    }
}

pub enum Node {
    //Dyadic
    Addition(~Token, ~Node, ~Node),
    Subtraction(~Token, ~Node, ~Node),
    Multiplication(~Token, ~Node, ~Node),
    Division(~Token, ~Node, ~Node),
    Maximum(~Token, ~Node, ~Node),
    Minimum(~Token, ~Node, ~Node),
    Power(~Token, ~Node, ~Node),

    //Monadic
    Conjugate(~Token, ~Node),
    Negate(~Token, ~Node),
    Reciprocal(~Token, ~Node),
    Sign(~Token, ~Node),
    Magnitude(~Token, ~Node),
    Ceiling(~Token, ~Node),
    Floor(~Token, ~Node),
    Exponential(~Token, ~Node),

    //Niladic
    Variable(~Token),
    Array(~[~Token]),
    Zilde(~Token),
}

impl EvalNode for Node {
    fn eval(&self) -> Result<~Value, ~str> {
        match self {
            &Addition(_, ref left, ref right) => eval_addition(*left, *right),
            &Subtraction(_, ref left, ref right) => eval_subtraction(*left, *right),
            &Multiplication(_, ref left, ref right) => eval_multiplication(*left, *right),
            &Division(_, ref left, ref right) => eval_division(*left, *right),
            &Maximum(_, ref left, ref right) => eval_maximum(*left, *right),
            &Minimum(_, ref left, ref right) => eval_minimum(*left, *right),
            &Power(_, ref left, ref right) => eval_power(*left, *right),

            &Conjugate(_, ref left) => eval_conjugate(*left),
            &Negate(_, ref left) => eval_negate(*left),
            &Reciprocal(_, ref left) => eval_reciprocal(*left),
            &Sign(_, ref left) => eval_sign(*left),
            &Magnitude(_, ref left) => eval_magnitude(*left),
            &Ceiling(_, ref left) => eval_ceiling(*left),
            &Floor(_, ref left) => eval_floor(*left),
            &Exponential(_, ref left) => eval_exponential(*left),

            _ => Err(~"Not yet implemented")
        }
    }
}

fn token_string(token: &Token) -> ~str {
    match token {
        &tokenizer::Number(ref token_data) => token_data.string.clone(),
        &tokenizer::Newline(ref token_data) => token_data.string.clone(),
        &tokenizer::String(ref token_data) => token_data.string.clone(),
        &tokenizer::Primitive(ref token_data) => token_data.string.clone(),
        &tokenizer::Variable(ref token_data) => token_data.string.clone(),
        &tokenizer::EndOfFile() => ~"(none)"
    }
}

pub fn node_to_string(node: &Node) -> ~str {
    format!("{:?}", node)
}
