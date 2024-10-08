pub mod cd;

pub struct Builtins;

impl Builtins {
    pub fn cd(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        return cd::cd(path);
    }
}
