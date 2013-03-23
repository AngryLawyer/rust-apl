use core::io::ReaderUtil;
use eval::Evaluator;
use nodes;

fn main() {
   io::println(~"Rust-APL version 0.0.1");
   let reader = io::stdin();
   loop {
        let read = reader.read_line();
        let mut eval = Evaluator::new(read);
        match eval.eval() {
            result::Ok(result) => {
                io::println(~"LOL");
            },
            result::Err(msg) => {
                io::println(msg);
            }
        }
   }
}
