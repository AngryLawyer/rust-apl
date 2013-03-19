pub enum Node {
    pub Niladic(~[Tokens]),
    pub Monadic(~[Tokens], Node),
    pub Dyadic(~[Tokens], Node, Node)
}

pub struct Parser {
    tokenizer: ~tokenizer::Tokenizer,
    parse_tree: ~Node
}

impl Parser {

}
