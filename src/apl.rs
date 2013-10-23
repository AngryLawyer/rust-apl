use std::io::ReaderUtil;
use eval::eval::Evaluator;
use eval::eval::Printable;
use std::io;

#[main]
fn main() {
   io::println("Rust-APL version 0.0.1");
   let reader = io::stdin();
   loop {
        let read = reader.read_line();
        let mut eval = Evaluator::new(read);
        match eval.eval() {
            Ok(result) => {
                io::println(result.to_string());
            },
            Err(msg) => {
                io::println(format!("Error: {}", msg));
            }
        }
   }
}
