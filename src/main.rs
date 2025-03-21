#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};
use std::env;
use std::path::Path;
use std::fs;
use regex::Regex;

fn parse_input(input: &str) -> Vec<String> {
    let re = Regex::new(r"'([^']*)'|(\S+)").unwrap();
    let mut args = Vec::new();

    for cap in re.captures_iter(input) {
        if let Some(quoted) = cap.get(1) {
            args.push(quoted.as_str().to_string());
        } else if let Some(unquoted) = cap.get(2) {
            args.push(unquoted.as_str().to_string());
        }
    }

    args
}

fn main() {
    let builtins = ["echo", "exit", "type", "pwd", "cd"];

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

        let args = parse_input(input);
        if args.is_empty() {
            continue;
        }

        let command = &args[0];
        let command_args = &args[1..];

        match command.as_str() {
            "exit" if command_args.get(0) == Some(&"0".to_string()) => break,

            "echo" => {
                println!("{}", command_args.join(" "));
            }

            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(_) => println!("pwd: error retrieving current directory"),
                }
            }

            "cd" => {
                if let Some(dir) = command_args.get(0) {
                    let path = if dir == "~" {
                        env::var("HOME").unwrap_or_else(|_| String::from("/"))
                    } else {
                        dir.to_string()
                    };

                    let path = Path::new(&path);
                    
                    if path.is_absolute() || path.starts_with(".") || path.starts_with("..") {
                        if let Err(_) = env::set_current_dir(path) {
                            println!("cd: {}: No such file or directory", dir);
                        }
                    } else {
                        println!("cd: currently only absolute, relative paths, and ~ are supported.");
                    }
                } else {
                    println!("cd: missing argument");
                }
            }

            "type" => {
                if let Some(cmd) = command_args.get(0) {
                    if builtins.contains(&cmd.as_str()) {
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
                match Command::new(command).args(command_args).spawn() {
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
