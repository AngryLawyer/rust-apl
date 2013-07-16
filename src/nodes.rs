use tokenizer;
use tokenizer::Token;

pub enum Node {
    //Dyadic
    pub Addition(@Token, ~Node, ~Node),
    pub Subtraction(@Token, ~Node, ~Node),
    pub Multiplication(@Token, ~Node, ~Node),

    //Monadic
    pub Conjugate(@Token, ~Node),
    pub Negate(@Token, ~Node),

    //Niladic
    pub Variable(@Token),
    pub Array(~[@Token]),
    pub Zilde(@Token),
}

fn token_string(token: &Token) -> ~str {
    match token {
        &tokenizer::Number(ref token_data) => copy token_data.string,
        &tokenizer::Newline(ref token_data) => copy token_data.string,
        &tokenizer::String(ref token_data) => copy token_data.string,
        &tokenizer::Primitive(ref token_data) => copy token_data.string,
        &tokenizer::Variable(ref token_data) => copy token_data.string,
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
        &Conjugate(ref _token, ref left) => {
            ~"CONJUGATE[ "+node_to_string(*left)+" ]"
        },
        &Negate(ref _token, ref left) => {
            ~"NEGATE[ "+node_to_string(*left)+" ]"
        },
        &Variable(ref token) => {
            ~"VAR("+token_string(*token)+")"
        },
        &Array(ref tokens) => {
            let mut string = ~"ARRAY(";
            for tokens.iter().advance |token| {
                string = string.append(token_string(*token)).append(",");
            }
            string.append(")")
        },
        &Zilde(ref token) => {
            ~"ZILDE("+token_string(*token)+")"
        }
    }
}
