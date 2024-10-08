use std::collections::HashMap;

use crate::env;

type FnPointer = fn(Vec<&str>) -> Result<(), Box<dyn std::error::Error>>;

fn back(arguments: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    env::chdir(&env::get_oldpwd());

    return Ok(());
}

fn home(arguments: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
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
