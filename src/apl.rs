use eval::eval::Evaluator;
use eval::eval::Printable;
use std::rt::io;

#[main]
fn main() {
   println!("Rust-APL version 0.0.1");
   let mut reader = io::buffered::BufferedReader::new(io::stdin());
   loop {
        match reader.read_line() {
            Some(read) => {
                let mut eval = Evaluator::new(read);
                match eval.eval() {
                    Ok(result) => {
                        println!("{}", result.to_string());
                    },
                    Err(msg) => {
                        println!("Error: {}", msg);
                    }
                }
            },
            None => ()
        }
   }
}
