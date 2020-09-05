#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
use std::env;

// These are required because other files use them!
// more here: http://www.sheshbabu.com/posts/rust-module-system/
mod lox;
mod scanner;
mod token;
use scanner::Scanner;

fn main() {
    // env_logger::init();
    let args: Vec<String> = env::args().collect();
    let _ = match lox::run_lox(args) {
        Ok(a) => a,
        Err(e) => panic!("Error in main: {}", e),
    };
}
