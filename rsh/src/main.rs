use std::{sync::{Arc, Mutex}};

mod execute;
mod prompt;
mod signals;

pub mod env;
pub mod builtins;
pub mod lib;

fn launcher(input: String, builtins: &builtins::Builtins) -> bool {
    let command: &str;
    let split: Vec<&str> = input.split_whitespace().collect();
    let mut arguments: Vec<&str> = Vec::new();

    if !split.is_empty() {
        command = split[0];
        if split.len() >= 2 {
            arguments = split[1..].to_vec();
        }

        return execute::execute(command, arguments, builtins);
    }

    return false;
}

fn main() {
    let exit_flag = Arc::new(Mutex::new(false));
    let exit_flag_clone: std::sync::Arc<std::sync::Mutex<bool>> = Arc::clone(&exit_flag);
    
    let _: Result<(), Box<dyn std::error::Error>> = signals::handle(exit_flag_clone);

    while !*exit_flag.lock().unwrap() {
        let input = prompt::prompt();
        *exit_flag.lock().unwrap() = launcher(input, &builtins::Builtins);
    }
}
