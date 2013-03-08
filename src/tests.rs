
mod tokenizer {

    use tokenizer;
    use tokenizer::Tokenizer;

    #[test]
    fn test_tokenize_one() {
        //Numbers
        for ([~"1", ~"321", ~"3.21", ~".21", ~"0.21", ~"¯321"]).each |number| {
            let mut tokenizer = Tokenizer::new(copy *number);
            match tokenizer.read_next_token() {
                result::Ok(tokenizer::Number(tokenData)) => {
                    //Pass
                    fail_unless!(tokenData.string == *number);
                    fail_unless!(tokenData.row == 0);
                    fail_unless!(tokenData.col == 0);
                },
                _ => {
                    fail!(~"Expected number");
                }
            }
        }
        //Offset number
        /*let mut tokenizer = Tokenizer::new(~" 123");
        match tokenizer.read_next_token() {
            result::Ok(tokenizer::Number(tokenData)) => {
                //Pass
                fail_unless!(tokenData.string == ~"123");
                fail_unless!(tokenData.row == 0);
                fail_unless!(tokenData.col == 1);
            },
            _ => {
                fail!(~"Expected number");
            }
        }*/


        //Strings
        /*let string = ~"'This is a string'";

        //Operators
        let string = ~"⍒";
        let string = ~"÷";

        //Primitive Operators

        //Comments
        let string = ~"⍝";
        let string = ~"⍝ lol";

        //Function definition
        let string = ~"∇Function";
        let string = ~"∇Function B";
        let string = ~"∇A Function B";
        let string = ~"∇A Function B;VAR";

        //Close function
        let string = ~"∇";

        //Endlines
        let string = ~"\n";

        //Variables
        let string = ~"⍙var";
        let string = ~"∆var";
        let string = ~"var";

        //Control
        let string = ~":If";
        let string = ~":Return";

        //Label
        let string = ~"LABEL:";
        //Todo- array indexing
        //System variables
        let string = ~"⎕SI";*/
    }
}
