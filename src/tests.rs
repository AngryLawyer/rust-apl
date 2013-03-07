
mod tokenizer {

    #[test]
    fn test_tokenize_one() {
        //Numbers
        let string = ~"1";
        let string = ~"321";

        //Strings
        let string = ~"'This is a string'";

        //Operators
        let string = ~"⍒";
        let string = ~"÷";

        //Comments
        let string = ~"⍝";
        let string = ~"⍝ lol";

        //Function definition
        let string = ~"∇Function";

        //Close function
        let string = ~"∇";

        //Endlines
        let string = ~"\n";

        //Variables
        let string = ~"⍙var";
        let string = ~"∆var";
        let string = ~"var";
    }
}
