#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let builtins = ["echo", "exit", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];

        match command {
            "exit" => break,
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                if args.is_empty() {
                    println!("type: missing argument");
                    continue;
                }
                let command_to_check = args[0];
                if builtins.contains(&command_to_check) {
                    println!("{} is a shell builtin", command_to_check);
                } else {
                    // Search for the command in PATH
                    let path = env::var("PATH").unwrap_or_default();
                    let mut found = false;

                    for dir in path.split(':') {
                        let full_path = Path::new(dir).join(command_to_check);
                        if full_path.exists() && fs::metadata(&full_path).unwrap().is_file() {
                            println!("{} is {}", command_to_check, full_path.display());
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        println!("{}: not found", command_to_check);
                    }
                }
            }
            _ => {
                // Search for the command in PATH
                let path = env::var("PATH").unwrap_or_default();
                let mut found = false;

                for dir in path.split(':') {
                    let full_path = Path::new(dir).join(command);
                    if full_path.exists() && fs::metadata(&full_path).unwrap().is_file() {
                        // Execute the external program
                        let output = process::Command::new(full_path)
                            .args(args)
                            .output()
                            .expect("Failed to execute command");

                        // Print the output of the external program
                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}