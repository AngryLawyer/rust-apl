use parser;
use parser::Parser;

#[test]
fn test_parse_number() {
   
    let number = ~"3.141"; //Everyone's favourite number 
    let mut parser = Parser::new(number);
    let parse_tree = parser.parse();
    match parse_tree {
        result::Ok(tree) => {
        },
        result::Err(msg) => {
            fail!(msg);
        }
    }
}

#[test]
fn test_parse_array() {
}
