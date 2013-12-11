use std::result;
use tokenizer;
use tokenizer::Tokenizer;
use test_utils::test_assert;

#[test]
fn test_tokenize_number() {
    let list = ~[~"1", ~"321", ~"3.21", ~".21", ~"0.21", ~"¯321"];
    for number in list.iter() {
        let mut tokenizer = Tokenizer::new(number.clone());
        match tokenizer.read_next_token() {
            result::Ok(~tokenizer::Number(tokenData)) => {

                //Pass
                test_assert(tokenData.string == *number, format!("Read {} expected {} ", tokenData.string, *number));
                /*fail_unless!(tokenData.row == 0);
                fail_unless!(tokenData.col == 0);*/
            },
            result::Err(msg) => {
                fail!(format!("Expected {} - {}", *number, msg));
            },
            _ => {
                fail!(format!("Unexpected token type for {}", *number));
            }
        }
    }

    //Complex number
    let list = ~[~"1J2", ~"0J21", ~"3.2J2.1", ~"¯321J¯321"];
    for number in list.iter() {
        let mut tokenizer = Tokenizer::new(number.clone());
        match tokenizer.read_next_token() {
            result::Ok(~tokenizer::Number(tokenData)) => {

                //Pass
                test_assert(tokenData.string == *number, format!("Read {} expected {} ", tokenData.string, *number));
                /*fail_unless!(tokenData.row == 0);
                fail_unless!(tokenData.col == 0);*/
            },
            result::Err(msg) => {
                fail!(format!("Expected {} - {}", *number, msg));
            },
            _ => {
                fail!(format!("Unexpected token type for {}", *number));
            }
        }
    }
    //Offset number
    let mut tokenizer = Tokenizer::new(~" 123⍝ lol");
    let expected = ~"123";
    match tokenizer.read_next_token() {
        result::Ok(~tokenizer::Number(tokenData)) => {
            //Pass
            test_assert(tokenData.string == expected, format!("Read {} expected {} ", tokenData.string, expected));
            /*fail_unless!(tokenData.row == 0);
            fail_unless!(tokenData.col == 1);*/
        },
        result::Err(msg) => {
            fail!(format!("Expected {} - {}", expected, msg));
        },
        _ => {
            fail!(format!("Unexpected token type for {}", expected));
        }
    }

    //Invalid numbers
    let list = [~".3.21", ~"3.2.1", ~"1.", ~".", ~"JJ", ~"1J", ~"J1", ~"0J1¯"];
    for number in list.iter() {
        let mut tokenizer = Tokenizer::new(number.clone());
        match tokenizer.read_next_token() {
            result::Ok(~tokenizer::Number(tokenData)) => {
                fail!(format!("Unexpectedly read {} from source {}",tokenData.string, *number));
            },
            result::Ok(_) => {
            },
            result::Err(_msg) => {
            }
        }
    }
}

#[test]
fn test_tokenize_newlines() {
    let list = [~"\n", ~"  \n", ~"\n\n", ~"⍝ lol\n", ~"\r", ~"\r\n", ~"\r\r"];
    for newline in list.iter() {
        let mut tokenizer = Tokenizer::new(newline.clone());
        match tokenizer.read_next_token() {
            result::Ok(~tokenizer::Newline(_tokenData)) => {
                //Pass
                /*fail_unless!(tokenData.row == 0);
                fail_unless!(tokenData.col == 0);*/
            },
            result::Err(msg) => {
                fail!(format!("Expected newline - {}", msg));
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
    let list = [(~"'Hello'", ~"Hello"),
        (~"\"Double quotes\"", ~"Double quotes"),
        (~"'Anything ⍝ lol'", ~"Anything ⍝ lol"),
        (~"'Inner \"\" quotes'", ~"Inner \"\" quotes"),
        (~"\"Inner '' quotes\"", ~"Inner '' quotes"),
        (~"'Escaped '' quote'", ~"Escaped ' quote"),
        (~"\"Escaped \"\" quote\"", ~"Escaped \" quote"),
        (~"\"Not Escaped '' quote\"", ~"Not Escaped '' quote"),
        (~"'Not Escaped \"\" quote'", ~"Not Escaped \"\" quote")
    ];
    for &(ref string, ref result) in list.iter() {
        let mut tokenizer = Tokenizer::new(string.clone());
        match tokenizer.read_next_token() {
            result::Ok(~tokenizer::String(tokenData)) => {
                test_assert(tokenData.string == *result, format!("Read {} expected {} ", tokenData.string, *result));

                //Pass
                /*fail_unless!(tokenData.row == 0);
                fail_unless!(tokenData.col == 0);*/
            },
            result::Err(msg) => {
                fail!(format!("Expected string for {} - {}", tokenizer.char_reader.source, msg));
            },
            _ => {
                fail!(~"Unexpected token type");
            }
        }
    }
}

#[test]
fn test_tokenize_primitives() {
    let list = [~"+", 
        ~"−",
        ~"×",
        ~"÷",
        ~"⌈",
        ~"⌊",
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
        ~"∘",
        ~"¨",
        ~"[",
        ~"]",
        ~"⍬",
        ~"⋄",
        ~"∇",
        ~"⍫",
        ~"(",
        ~")",
        ~"←",
        ~"{",
        ~"}",
        ~"⍵",
        ~"."
    ];

    for prim in list.iter() {
        let mut tokenizer = Tokenizer::new(prim.clone());

        match tokenizer.read_next_token() {
            result::Ok(~tokenizer::Primitive(tokenData)) => {
                test_assert(tokenData.string == *prim, format!("Read {} expected {} ", tokenData.string, *prim));
                /*fail_unless!(tokenData.row == 0);
                fail_unless!(tokenData.col == 0);*/
            },
            result::Err(msg) => {
                fail!(format!("Expected primitive for {} - {}", *prim, msg));
            },
            _ => {
                fail!(format!("Unexpected token type for {}", *prim));
            }
        }
    }
}

#[test]
fn test_tokenize_variables() {
    //Standard Variables 
    let list = [(~"Hello", ~"Hello"),
        (~"hi", ~"hi"),
        (~"HOLA⍝comment", ~"HOLA"),
        (~"∆delta", ~"∆delta"),
        (~"⍙delta", ~"⍙delta")
    ];

    for &(ref string, ref result) in list.iter() {
        let mut tokenizer = Tokenizer::new(string.clone());
        match tokenizer.read_next_token() {
            result::Ok(~tokenizer::Variable(tokenData)) => {
                test_assert(tokenData.string == *result, format!("Read {} expected {} ", tokenData.string, *result));

                //Pass
                /*fail_unless!(tokenData.row == 0);
                fail_unless!(tokenData.col == 0);*/
            },
            result::Err(msg) => {
                fail!(format!("Expected variable for {} - {}", tokenizer.char_reader.source, msg));
            },
            _ => {
                fail!(~"Unexpected token type");
            }
        }
    }
}

#[test]
fn test_tokenize_multiple() {
    let string = ~"life←{↑1 ⍵∨.∧3 4=+/,¯1 0 1∘.⊖¯1 0 1∘.⌽⊂⍵}";
    let mut tokenizer = Tokenizer::new(string);
    let mut tokens = ~[];
    loop {
        match tokenizer.read_next_token() {
            result::Ok(token) => {
                match token {
                    ~tokenizer::EndOfFile => {
                        break;
                    },
                    _ => {
                        tokens.push(token);
                    }
                }
            },
            result::Err(msg) => {
                fail!(msg);
            }
        }
    }
    test_assert(tokens.len() == 28, format!("Expected 28 tokens, got {}", tokens.len()));
}
/*
TODO:
//Control
let string = ~":If";
let string = ~":Return";
//Label
let string = ~"LABEL:";
//System variables
let string = ~"⎕SI";*/
