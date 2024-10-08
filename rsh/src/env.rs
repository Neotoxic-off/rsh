use std::env;
use std::path::Path;

fn get_value(key: &str) -> String {
    return env::var(key).unwrap();
}

pub fn get_pwd() -> String {
    return get_value("PWD");
}

pub fn get_home() -> String {
    return get_value("HOME");
}

pub fn get_oldpwd() -> String {
    return get_value("OLDPWD");
}

pub fn chdir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let new_path = Path::new(path);

    env::set_current_dir(new_path)?;

    return Ok(());
}
