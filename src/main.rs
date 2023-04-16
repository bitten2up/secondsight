use std::io::{self, Read};
use secondsight;

fn main() {

    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read from stdin");

    let output = secondsight::secondsightify(&input);
    println!("{}", output);
}