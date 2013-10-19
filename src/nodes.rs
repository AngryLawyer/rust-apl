use tokenizer;
use tokenizer::Token;

pub enum Node {
    //Dyadic
    Addition(@Token, ~Node, ~Node),
    Subtraction(@Token, ~Node, ~Node),
    Multiplication(@Token, ~Node, ~Node),
    Division(@Token, ~Node, ~Node),
    Maximum(@Token, ~Node, ~Node),
    Minimum(@Token, ~Node, ~Node),

    //Monadic
    Conjugate(@Token, ~Node),
    Negate(@Token, ~Node),
    Reciprocal(@Token, ~Node),
    Sign(@Token, ~Node),
    Magnitude(@Token, ~Node),
    Ceiling(@Token, ~Node),
    Floor(@Token, ~Node),

    //Niladic
    Variable(@Token),
    Array(~[@Token]),
    Zilde(@Token),
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
