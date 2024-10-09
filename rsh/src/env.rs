use std::env::{self, VarError};
use std::path::{Path, PathBuf};

static PWD: &str = "PWD";
static OLDPWD: &str = "OLDPWD";
static HOME: &str = "HOME";
static PATH: &str = "PATH";

fn get_value(key: &str) -> String {
    let result: Result<String, VarError> = env::var(key);

    match result {
        Ok(_) => return result.unwrap(),
        Err(_) => return String::new()
    }
}

fn set_value(key: &str, value: &str) -> () {
    env::set_var(key, value);
}

pub fn get_path() -> Vec<String> {
    let path = env::var(PATH).unwrap_or_else(|_| String::new());
    path.split(':').map(|s| s.to_string()).collect()
}

pub fn get_pwd() -> String {
    return get_value(PWD);
}

pub fn get_home() -> String {
    return get_value(HOME);
}

pub fn get_oldpwd() -> String {
    return get_value(OLDPWD);
}

pub fn set_oldpwd(value: &str) -> () {
    set_value(OLDPWD, value);
}

pub fn set_pwd(value: &str) -> () {
    set_value(PWD, value);
}

pub fn chdir(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let new_path = Path::new(path);

    env::set_current_dir(new_path)?;
    let current_dir: PathBuf = env::current_dir().unwrap();

    if let Some(current_dir_str) = current_dir.to_str() {
        set_pwd(current_dir_str);
    } else {
        eprintln!("Failed to convert current directory to a valid string.");
    }

    return Ok(());
}
