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
    
    let re = Regex::new(r"cat\s+((?:'[^']*'\s*)+)").unwrap();
    if let Some(caps) = re.captures(input) {
        let mut ss = vec!["cat"];
        let inner_re = Regex::new(r"'([^']*)'").unwrap();
        inner_re.captures_iter(caps.get(1).unwrap().as_str()).for_each(|cap| {
            ss.push(cap.get(1).unwrap().as_str());
        });
        return ss;
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
            [program, arg1, ..] => {
                let mut child = process::Command::new(program);
                let mut args: Vec<&str> = vec![arg1];
                args.append(&mut tok[2..].to_vec());
                child.args(&args);
                child.spawn().unwrap().wait().unwrap();
            },
            _ => println!("{}: not found", input.trim())
        }
    }
}
