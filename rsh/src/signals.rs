use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
use std::{error::Error, sync::{Arc, Mutex}, thread};

pub fn handle(exit_flag: Arc<Mutex<bool>>) -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new([SIGINT, SIGTERM])?;

    thread::spawn(move || {
        for _ in signals.forever() {
            println!();
            let mut exit: std::sync::MutexGuard<'_, bool> = exit_flag.lock().unwrap();
            *exit = true;
        }
    });

    Ok(())
}
