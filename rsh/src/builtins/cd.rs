use std::env;
use std::path::Path;

pub fn cd(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let new_path = Path::new(path);

    env::set_current_dir(new_path)?;

    return Ok(());
}
