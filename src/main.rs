#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let builtins = ["echo", "exit", "type", "pwd"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" if args.get(0) == Some(&"0") => break,
            "echo" => {
                println!("{}", args.join(" "));
            }
            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(_) => println!("pwd: error retrieving current directory"),
                }
            }
            "type" => {
                if let Some(cmd) = args.get(0) {
                    if builtins.contains(cmd) {
                        println!("{} is a shell builtin", cmd);
                    } else {
                        let path = env::var("PATH").unwrap_or_default();
                        let mut found = false;

                        for dir in path.split(':') {
                            let full_path = Path::new(dir).join(cmd);
                            if full_path.exists() && fs::metadata(&full_path).unwrap().is_file() {
                                println!("{} is {}", cmd, full_path.display());
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            println!("{}: not found", cmd);
                        }
                    }
                } else {
                    println!("Usage: type <command>");
                }
            }
            _ => {
                // Try executing an external command
                match Command::new(command).args(&args).spawn() {
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(_) => {
                        println!("{}: command not found", command);
                    }
                }
            }
        }
    }
}
