mod execute;
mod prompt;

pub mod env;
pub mod builtins;
pub mod lib;

fn launcher(input: String, builtins: &builtins::Builtins) -> bool {
    let command: &str;
    let split: Vec<&str>;
    let mut arguments: Vec<&str> = Vec::new();

    split = input.split_whitespace().collect();
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
    let mut exit: bool = false;
    let mut input: String;
    let builtins: builtins::Builtins = builtins::Builtins;

    while exit != true {
        input = prompt::prompt();
        exit = launcher(input, &builtins);
    }
}
