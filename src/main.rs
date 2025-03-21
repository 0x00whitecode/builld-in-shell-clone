#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};
use std::env;
use std::path::Path;
use std::fs;

fn parse_input(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quotes = false;

    for c in input.chars() {
        match c {
            '\'' if !in_single_quotes => in_single_quotes = true, // Enter single quotes
            '\'' if in_single_quotes => in_single_quotes = false, // Exit single quotes
            ' ' if !in_single_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            _ => current_arg.push(c),
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

fn main() {
    let builtins = ["echo", "exit", "type", "pwd", "cd"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
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

                    if let Err(_) = env::set_current_dir(Path::new(&path)) {
                        println!("cd: {}: No such file or directory", dir);
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
