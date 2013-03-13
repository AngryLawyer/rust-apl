mod tokenizer {

    fn test_assert(value: bool, message: ~str) {
        if !value {
            fail!(message);
        }
    }

    use tokenizer;
    use tokenizer::Tokenizer;

    fn test_tokenize() {
        test_tokenize_number();
        test_tokenize_newlines();
        test_tokenize_strings();
    }

    #[test]
    fn test_tokenize_number() {
        for ([~"1", ~"321", ~"3.21", ~".21", ~"0.21", ~"¯321"]).each |number| {
            let mut tokenizer = Tokenizer::new(copy *number);
            match tokenizer.read_next_token() {
                result::Ok(tokenizer::Number(tokenData)) => {

                    //Pass
                    test_assert(tokenData.string == *number, fmt!("Read %s expected %s ", tokenData.string, *number));
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
        let mut tokenizer = Tokenizer::new(~" 123⍝ lol");
        let expected = ~"123";
        match tokenizer.read_next_token() {
            result::Ok(tokenizer::Number(tokenData)) => {
                //Pass
                test_assert(tokenData.string == expected, fmt!("Read %s expected %s ", tokenData.string, expected));
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
                result::Err(_msg) => {
                },
                _ => {
                    fail!(~"Unexpected token type");
                }
            }
        }
        /*
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

    #[test]
    fn test_tokenize_newlines() {
        for ([~"\n", ~"  \n", ~"\n\n", ~"⍝ lol\n", ~"\r", ~"\r\n", ~"\r\r"]).each |newline| {
            let mut tokenizer = Tokenizer::new(copy *newline);
            match tokenizer.read_next_token() {
                result::Ok(tokenizer::Newline(tokenData)) => {
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

    #[test]
    fn test_tokenize_strings() {
        //Standard strings
        for ([(~"'Hello'", ~"Hello"),
              (~"\"Double quotes\"", ~"Double quotes"),
              (~"'Anything ⍝ lol'", ~"Anything ⍝ lol"),
              (~"'Inner \"\" quotes'", ~"Inner \"\" quotes"),
              (~"\"Inner '' quotes\"", ~"Inner '' quotes"),
              (~"'Escaped '' quote'", ~"Escaped ' quote"),
              (~"\"Escaped \"\" quote\"", ~"Escaped \" quote"),
              (~"\"Not Escaped '' quote\"", ~"Not Escaped '' quote"),
              (~"'Not Escaped \"\" quote'", ~"Not Escaped \"\" quote")
              ]).each |&(string, result)| {
            let mut tokenizer = Tokenizer::new(string);
            match tokenizer.read_next_token() {
                result::Ok(tokenizer::String(tokenData)) => {
                    test_assert(tokenData.string == result, fmt!("Read %s expected %s ", tokenData.string, result));

                    //Pass
                    fail_unless!(tokenData.row == 0);
                    fail_unless!(tokenData.col == 0);
                },
                result::Err(msg) => {
                    fail!(fmt!("Expected string for %s - %s", tokenizer.char_reader.source, msg));
                },
                _ => {
                    fail!(~"Unexpected token type");
                }
            }
        }
    }

    #[test]
    fn test_tokenize_primitives() {
        for ([~"+", 
              ~"−",
              ~"×",
              ~"÷",
              ~"⌈",
              ~"⌉",
              ~"∣",
              ~"⍳",
              ~"?",
              ~"⋆",
              ~"⍟",
              ~"○",
              ~"!",
              ~"⌹",
              ~"<",
              ~"≤",
              ~"=",
              ~"≥",
              ~">",
              ~"≠",
              ~"≡",
              ~"≢",
              ~"∊",
              ~"⍷",
              ~"∪",
              ~"∩",
              ~"~",
              ~"∨",
              ~"∧",
              ~"⍱",
              ~"⍲",
              ~"⍴",
              ~",",
              ~"⍪",
              ~"⌽",
              ~"⊖",
              ~"⍉",
              ~"↑",
              ~"↓",
              ~"⊂",
              ~"⊃",
              ~"⌷",
              ~"⍋",
              ~"⍒",
              ~"⊤",
              ~"⊥",
              ~"⍺",
              ~"⍕",
              ~"⍎",
              ~"⊣",
              ~"⊢",
              ~"▯",
              ~"⍞",
              ~"/",
              ~"⌿",
              ~"\\",
              ~"⍀",
              ~"⌿",
              ~"∘.",
              ~"¨",
              ~"[", //FIXME: Check that this should be parsed here
              ~"]",
              ~"⍬",
              ~"⋄", //FIXME: A type of newline?
              ~"∇", //FIXME: Function definition?
              ~"⍫",
              ~"(",
              ~")"
              ]).each |newline| {
            let mut tokenizer = Tokenizer::new(copy *newline);
            match tokenizer.read_next_token() {
                result::Ok(tokenizer::Primitive(tokenData)) => {
                    //FIXME: Check it's the same primitive!
                    //Pass
                    fail_unless!(tokenData.row == 0);
                    fail_unless!(tokenData.col == 0);
                },
                result::Err(msg) => {
                    fail!(fmt!("Expected primitive - %s", msg));
                },
                _ => {
                    fail!(~"Unexpected token type");
                }
            }
        }
    }

    #[test]
    fn test_tokenize_brackets() {
    }
}
