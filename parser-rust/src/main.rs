use parser::Scanner;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read input");

    let mut scanner = Scanner::new(input);
    scanner.scan_tokens().iter().for_each(|t| println!("{}", t))
}
