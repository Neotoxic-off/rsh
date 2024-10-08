use std::env;

pub fn set(key: &str, value: &str) -> () {
    env::set_var(key, value);
}
