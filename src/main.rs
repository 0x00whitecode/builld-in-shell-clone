#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::env;
use std::path::Path;
use std::fs;


fn main() {
    let builtins = ["echo", "exit", "type"];
    // Uncomment this block to pass the first stage
   loop {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();


    match input.trim() {
        "exit 0" => break,
        // add the echo to this 
        input if input.starts_with("echo ") => println!("{}", &input[5..]),
        input if input.starts_with("type ") => {
            let command = &input[5..];
            if builtins.contains(&command){
                println!("{} is a shell builtin", command);
            }else {
                // Search for the command in PATH
                let path = env::var("PATH").unwrap_or_default();
                let mut found = false;

                for dir in path.split(':') {
                    let full_path = Path::new(dir).join(command);
                    if full_path.exists() && fs::metadata(&full_path).unwrap().is_file() {
                        println!("{} is {}", command, full_path.display());
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("{}: not found", command);
                }
            }
        }
        &_ => {
            println!("{}: command not found", input.trim());
        }
   }
   input.clear();
}
}
