use clap::Parser;
use std::io::{self, Write};

mod commands;

/// TerminalAI: A Local Code Interpreter in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();
    println!("Welcome to TerminalAI! Type your commands below. Type 'exit' to quit.");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            println!("Error reading input. Exiting.");
            break;
        }
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }
        match commands::handle_command(input) {
            Ok(Some(output)) => println!("{}", output),
            Ok(None) => println!("Unrecognized command."),
            Err(e) => println!("Error: {}", e),
        }
    }
} 