mod builtins; 

fn main() {
    let builtins: builtins::Builtins = builtins::Builtins;

    if let Err(e) = builtins.cd("fdsjksdf") {
        eprintln!("Error changing directory: {}", e);
    }
}
