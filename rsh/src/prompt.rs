use std::io::{self, Write};

use crate::env;

fn get_lowest_dir() -> String {
    let pwd: String = env::get_pwd();
    let separator: char = '/';
    let count: usize = pwd.chars().filter(|c| *c == separator).count();
    let split: Vec<&str>;
    if count > 1 {
        split = pwd.split(separator).collect();
        return String::from(split[count]);
    }

    return pwd;
}

fn ui() -> String {
    let pwd: String = get_lowest_dir();
    let prompt: String = format!("{pwd}$>");

    return prompt;
}

pub fn prompt() -> String {
    let mut input = String::new();

    print!("{}", ui());
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}
