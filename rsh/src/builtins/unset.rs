use std::env;

pub fn unset(key: &str) -> () {
    env::remove_var(key);
}
