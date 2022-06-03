use ansi_term::{Colour::*, Style};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

pub fn init() -> Result<(), SetLoggerError> {
    let pl = Logger::new();

    log::set_max_level(pl.max_level);
    log::set_boxed_logger(Box::new(pl))?;
    Ok(())
}

struct Logger {
    max_level: LevelFilter,
}

impl Logger {
    fn new() -> Logger {
        Logger {
            max_level: match std::env::var("RUST_LOG") {
                Ok(level) => match level.to_lowercase().as_str() {
                    "trace" => log::LevelFilter::Trace,
                    "debug" => log::LevelFilter::Debug,
                    "warn" => log::LevelFilter::Warn,
                    "error" => log::LevelFilter::Error,
                    _ => log::LevelFilter::Info,
                },
                Err(_) => log::LevelFilter::Info,
            },
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        &metadata.level().to_level_filter() <= &self.max_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => eprintln!("{} {}", Style::new().bold().fg(Red).paint("error:"), record.args()),
                Level::Warn  => eprintln!("{} {}", Style::new().bold().fg(Yellow).paint("warn: "), record.args()),
                Level::Info  => println! ("{} {}", Style::new().bold().fg(White).paint("info: "), record.args()),
                Level::Debug => println! ("{} {}", Style::new().bold().fg(Cyan).paint("debug:"), record.args()),
                Level::Trace => println! ("{} {}", Style::new().bold().fg(Purple).paint("trace:"), record.args()),
            }
        }
    }

    fn flush(&self) {}
}
