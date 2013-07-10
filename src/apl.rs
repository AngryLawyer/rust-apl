use std::io::ReaderUtil;
use eval::Evaluator;
use eval::Printable;
use std::io;
use std::result;

#[main]
fn main() {
   io::println("Rust-APL version 0.0.1");
   let reader = io::stdin();
   loop {
        let read = reader.read_line();
        let mut eval = Evaluator::new(read);
        match eval.eval() {
            result::Ok(result) => {
                io::println(result.to_string());
            },
            result::Err(msg) => {
                io::println(fmt!("Error: %s", msg));
            }
        }
   }
}
