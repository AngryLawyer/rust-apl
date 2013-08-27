use tokenizer;
use tokenizer::Token;

pub enum Node {
    //Dyadic
    Addition(@Token, ~Node, ~Node),
    Subtraction(@Token, ~Node, ~Node),
    Multiplication(@Token, ~Node, ~Node),
    Division(@Token, ~Node, ~Node),

    //Monadic
    Conjugate(@Token, ~Node),
    Negate(@Token, ~Node),
    Reciprocal(@Token, ~Node),
    Sign(@Token, ~Node),
    Magnitude(@Token, ~Node),

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
    match node {
        &Addition(ref _token, ref left, ref right) => {
            ~"ADDITION[ "+node_to_string(*left)+", "+node_to_string(*right)+" ]"
        },
        &Subtraction(ref _token, ref left, ref right) => {
            ~"SUBTRACTION[ "+node_to_string(*left)+", "+node_to_string(*right)+" ]"
        },
        &Multiplication(ref _token, ref left, ref right) => {
            ~"MULTIPLICATION[ "+node_to_string(*left)+", "+node_to_string(*right)+" ]"
        },
        &Division(ref _token, ref left, ref right) => {
            ~"DIVISION[ "+node_to_string(*left)+", "+node_to_string(*right)+" ]"
        },
        &Conjugate(ref _token, ref left) => {
            ~"CONJUGATE[ "+node_to_string(*left)+" ]"
        },
        &Negate(ref _token, ref left) => {
            ~"NEGATE[ "+node_to_string(*left)+" ]"
        },
        &Reciprocal(ref _token, ref left) => {
            ~"RECIPROCAL[ "+node_to_string(*left)+" ]"
        },
        &Sign(ref _token, ref left) => {
            ~"SIGN[ "+node_to_string(*left)+" ]"
        },
        &Magnitude(ref _token, ref left) => {
            ~"MAGNITUDE[ "+node_to_string(*left)+" ]"
        },
        &Variable(ref token) => {
            ~"VAR("+token_string(*token)+")"
        },
        &Array(ref tokens) => {
            let mut string = ~"ARRAY(";
            let token_items: ~[~str] = tokens.iter().map(|token| token_string(*token)).collect();
            string = string.append(token_items.connect(","));
            string.append(")")
        },
        &Zilde(ref token) => {
            ~"ZILDE("+token_string(*token)+")"
        }
    }
}
