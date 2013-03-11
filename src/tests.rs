
mod tokenizer {

    use tokenizer;
    use tokenizer::Tokenizer;

    #[test]
    fn test_tokenize() {
        test_tokenize_number();
        test_tokenize_newlines();
    }

    fn test_tokenize_number() {
        //Numbers
        for ([~"1", ~"321", ~"3.21", ~".21", ~"0.21", ~"¯321"]).each |number| {
            let mut tokenizer = Tokenizer::new(copy *number);
            match tokenizer.read_next_token() {
                result::Ok(tokenizer::Number(tokenData)) => {
                    io::println(fmt!("Read %s expected %s ", tokenData.string, *number));

                    //Pass
                    fail_unless!(tokenData.string == *number);
                    fail_unless!(tokenData.row == 0);
                    fail_unless!(tokenData.col == 0);
                },
                result::Err(msg) => {
                    fail!(fmt!("Expected number - %s", msg));
                },
                _ => {
                    fail!(~"Unexpected token type");
                }
            }
        }
        //Offset number
        let mut tokenizer = Tokenizer::new(~" 123");
        let expected = ~"123";
        match tokenizer.read_next_token() {
            result::Ok(tokenizer::Number(tokenData)) => {
                //Pass
                io::println(fmt!("Read %s expected %s ", tokenData.string, expected));
                fail_unless!(tokenData.string == expected);
                /*fail_unless!(tokenData.row == 0);
                fail_unless!(tokenData.col == 1);*/
            },
            result::Err(msg) => {
                fail!(fmt!("Expected number - %s", msg));
            },
            _ => {
                fail!(~"Unexpected token type");
            }
        }

        //Invalid numbers
        for ([~".3.21", ~"3.2.1", ~"1.", ~"."]).each |number| {
            let mut tokenizer = Tokenizer::new(copy *number);
            match tokenizer.read_next_token() {
                result::Ok(tokenizer::Number(tokenData)) => {
                    fail!(fmt!("Unexpectedly read %s from source %s",tokenData.string, *number));
                },
                result::Err(msg) => {
                    io::println(fmt!("Correctly got error %s from %s", msg, *number));
                },
                _ => {
                    fail!(~"Unexpected token type");
                }
            }
        }


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

    fn test_tokenize_newlines() {
        //Numbers
        for ([~"\n", ~"  \n", ~"\n\n"]).each |newline| {
            let mut tokenizer = Tokenizer::new(copy *newline);
            match tokenizer.read_next_token() {
                result::Ok(tokenizer::Newline(tokenData)) => {
                    io::println("Read newline");
                    //Pass
                    fail_unless!(tokenData.row == 0);
                    fail_unless!(tokenData.col == 0);
                },
                result::Err(msg) => {
                    fail!(fmt!("Expected newline - %s", msg));
                },
                _ => {
                    fail!(~"Unexpected token type");
                }
            }
        }
    }
}
