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
            ["type", command] => {
                match command {
                    "echo" => {println!("echo is a shell builtin")},
                    "exit" => {println!("exit is a shell builtin")},
                    "type" => {println!("type is a shell builtin")},
                    _ => println!("{}: not found", command)
                }
            },
            _ => println!("{}: not found", input.trim())
        }
    }
}
