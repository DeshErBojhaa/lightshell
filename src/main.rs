#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, process, vec};
use std::fs::read_dir;
use std::path::PathBuf;
use regex::Regex;

fn token(input: &str) -> Vec<&str> {
    let re = Regex::new(r"echo\s+'([^']*)'").unwrap();
    if let Some(caps) = re.captures(input) {
        let ss = caps.get(1).map_or("", |m| m.as_str());
        return vec!["echo", ss];
    }
    input.split_whitespace().collect()
}

fn handle_type(cmd : &str) {
    let path_var = env::var("PATH").unwrap();
    let path = path_var.split(':').collect::<Vec<_>>();
    for dir in path {
        if !PathBuf::from(dir).is_dir() { continue };
        if read_dir(dir).
            unwrap().
            any(|entry| entry.as_ref().unwrap().file_name().to_str().unwrap() == cmd)
            {
                println!("{cmd} is {dir}/{cmd}");
                return
            }
    }
    println!("{}: not found", cmd);
}


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let tok = token(input.trim_ascii());
        match tok[..] {
            ["exit", code] => {
                process::exit(code.parse::<i32>().unwrap());
            },
            ["echo", ..] => {
                let arg = tok[1..].join(" ");
                println!("{}", arg);
            },
            ["type", command] => match command {
                "echo" => {println!("echo is a shell builtin")},
                "exit" => {println!("exit is a shell builtin")},
                "type" => {println!("type is a shell builtin")},
                _ => handle_type(command)
            },
            [program, arg] => {
                let mut child = process::Command::new(program);
                child.arg(arg);
                child.spawn().unwrap().wait().unwrap();
            },
            _ => println!("{}: not found", input.trim())
        }
    }
}
