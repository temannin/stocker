mod scan;
mod user_config;
mod worker;

use std::thread;
use std::time::Duration;

use systemd_journal_logger::JournalLog;

fn main() {
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(log::LevelFilter::Info);

    thread::spawn(move || worker::start());

    loop {
        std::thread::sleep(Duration::from_secs(1))
    }
}
