#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn token(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let tok = token(input.trim());
        match tok[..] { 
            ["exit", code] => {
                process::exit(code.parse::<i32>().unwrap());
            },
            ["echo", ..] => println!("{}", tok[1..].join(" ")),
            _ => println!("{}: not found", input.trim())  
        }
    }
}
