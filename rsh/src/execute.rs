use crate::builtins;

pub fn execute(command: &str, arguments: Vec<&str>, builtins: &builtins::Builtins) -> bool {
    if command == "exit" {
        return true;
    }

    if let Some(handler) = builtins.handle(command) {
        let _ = handler(builtins, arguments);
    }

    return false;
}
