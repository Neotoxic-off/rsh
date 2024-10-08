use std::io;
use std::fmt;

use crate::env;

fn get_lowest_dir() -> String {
    let pwd: String = env::get_pwd();
    let separator: char = '/';
    let count: usize = pwd.chars().filter(|c| *c == separator).count();
    let mut split: Vec<&str>;
    if count > 1 {
        split = pwd.split(separator).collect();
        return String::from(split[count - 1]);
    }

    return pwd;
}

fn ui() -> String {
    let pwd: String = get_lowest_dir();
    let mut prompt: String = format!("{pwd}$>");

    return prompt;
}

pub fn prompt() -> String {
    let mut input: String = String::new();
    println!("{}",  ui());
    let _ = io::stdin().read_line(&mut input);

    return input.trim().to_string();
}
