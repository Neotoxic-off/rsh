use std::process::Command;

use crate::builtins;
use crate::env;
use crate::lib;

fn get_path_command(command: &str) -> String {
    let paths: Vec<String> = env::get_path();
    let mut content: Vec<String> = Vec::new();

    for path in paths.iter() {
        if lib::is_directory(path) == true {
            content = lib::get_directory(path, false);
            if content.iter().any(|e| command.contains(e)) {
                return path.to_string();
            }
        }
    }

    return String::new();
}

fn exec(command: &str, arguments: Vec<&str>) -> String {
    let result: String;
    let foo = Command::new(command)
                      .args(&arguments)  // Use `args` to pass the entire list of arguments
                      .output()
                      .expect("Failed to execute command");  // Handle the error gracefully

    if foo.status.success() {
        result = String::from_utf8_lossy(&foo.stdout).to_string();
    } else {
        result = String::from_utf8_lossy(&foo.stderr).to_string();
    }

    return result;
}

pub fn execute(command: &str, arguments: Vec<&str>, builtins: &builtins::Builtins) -> bool {
    let mut command_path: String = String::new();
    let mut result: String;
    
    if command == "exit" {
        return true;
    }

    if let Some(handler) = builtins.handle(command) {
        let _ = handler(builtins, arguments);
    } else {
        command_path = get_path_command(command);
        if command_path.is_empty() == false {
            result = exec(&format!("{command_path}/{command}"), arguments);
            println!("{}", result);
        }
    }

    return false;
}
