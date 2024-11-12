mod scan;
mod user_config;
mod worker;

use std::thread;
use std::time::Duration;

use slog::{o, Drain, Logger};
use slog_scope::info;

fn main() {
    let decorator = slog_term::PlainDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, o!());
    let _guard = slog_scope::set_global_logger(logger);

    info!("Starting stocker {:?}", env!("CARGO_PKG_VERSION"));

    thread::spawn(move || worker::start());

    loop {
        std::thread::sleep(Duration::from_secs(1))
    }
}
