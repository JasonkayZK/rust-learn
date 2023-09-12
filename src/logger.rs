use std::env;

use log::{Level, LevelFilter, Metadata, Record};

struct Logger;

static LOG_LEVEL: &str = "LOG_LEVEL";

pub fn init() {
    static LOGGER: Logger = Logger;
    log::set_logger(&LOGGER).unwrap();

    let log_level: String = env::var(LOG_LEVEL).unwrap_or_else(|_| String::from("INFO"));
    log::set_max_level(match log_level.as_str() {
        "ERROR" => LevelFilter::Error,
        "WARN" => LevelFilter::Warn,
        "INFO" => LevelFilter::Info,
        "DEBUG" => LevelFilter::Debug,
        "TRACE" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    });
}

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };


        println!(
            "\u{1B}[{}m[{:>5}][{:>10}]:{} - {}\u{1B}[0m",
            color,
            record.level(),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.target(),
            record.args(),
        );
    }

    fn flush(&self) {}
}
