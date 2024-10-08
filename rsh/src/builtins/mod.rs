pub mod cd;
pub mod set;
pub mod unset;

pub struct Builtins;

type CommandHandler = fn(&Builtins, Vec<&str>) -> Result<(), Box<dyn std::error::Error>>;

impl Builtins {
    pub fn handle(&self, command: &str) -> Option<CommandHandler> {
        match command {
            "cd" => Some(Self::cd),
            "set" => Some(Self::set),
            "unset" => Some(Self::unset),
            _ => None
        }
    }

    pub fn cd(&self, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let mut path: &str = "~";
        let mut flags: Vec<&str> = Vec::new();

        if args.is_empty() == false {
            path = args[0];
            if args.len() > 1 {
                flags = args[1..].to_vec();
            }
        }

        cd::cd(path, flags)
    }

    pub fn set(&self, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let key = args[0];
        let value = args[1];

        set::set(key, value);

        Ok(())
    }

    pub fn unset(&self, args: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let key = args[0];

        unset::unset(key);

        Ok(())
    }
}
