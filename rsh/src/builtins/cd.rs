use std::{collections::HashMap, ptr::null};

use crate::env;

type FnPointer = fn(Vec<&str>) -> Result<(), Box<dyn std::error::Error>>;

fn back(arguments: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let oldpwd: String = env::get_oldpwd();

    if oldpwd.is_empty() == false {
        env::chdir(&oldpwd);
    }

    return Ok(());
}

fn home(arguments: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    env::set_oldpwd(&env::get_pwd());
    env::chdir(&env::get_home());

    return Ok(());
}

pub fn cd(path: &str, arguments: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let commands: HashMap<&str, FnPointer> = HashMap::from([
        ("-", back as FnPointer),
        ("~", home as FnPointer)
    ]);

    if let Some(&command) = commands.get(&path) {
        return command(arguments);
    }

    return Ok(());
}
