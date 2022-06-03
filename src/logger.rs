use std::io;
use std::io::Write;
use std::fmt::Arguments;

use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
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
                    "error" => log::LevelFilter::Error,
                    "info" => log::LevelFilter::Info,
                    _ => log::LevelFilter::Warn, // Only important messages by default
                },
                Err(_) => log::LevelFilter::Warn,
            },
        }
    }
    
    fn print(level: Level, args: &Arguments) -> io::Result<()> {
        let (bufwtr, mut buffer) = match level {
            Level::Error => {
                let bufwtr = BufferWriter::stderr(ColorChoice::Always);
                let mut buffer = bufwtr.buffer();
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
                write!(&mut buffer, "error:")?;
                (bufwtr, buffer)
            }
            Level::Warn  => {
                let bufwtr = BufferWriter::stderr(ColorChoice::Always);
                let mut buffer = bufwtr.buffer();
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))?;
                write!(&mut buffer, "warn:")?;
                (bufwtr, buffer)
            }
            Level::Info  => {
                let bufwtr = BufferWriter::stdout(ColorChoice::Always);
                let mut buffer = bufwtr.buffer();
                buffer.set_color(ColorSpec::new().set_bold(true))?;
                write!(&mut buffer, "info:")?;
                (bufwtr, buffer)
            }
            Level::Debug => {
                let bufwtr = BufferWriter::stdout(ColorChoice::Always);
                let mut buffer = bufwtr.buffer();
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
                write!(&mut buffer, "debug:")?;
                (bufwtr, buffer)
            }
            Level::Trace => {
                let bufwtr = BufferWriter::stdout(ColorChoice::Always);
                let mut buffer = bufwtr.buffer();
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)).set_bold(true))?;
                write!(&mut buffer, "trace:")?;
                (bufwtr, buffer)
            }
        };
       
       buffer.reset()?;
       writeln!(&mut buffer, " {}", args)?;
       bufwtr.print(&buffer)?;

       Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        &metadata.level().to_level_filter() <= &self.max_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if Logger::print(record.level(), record.args()).is_err() {
                println!("{}", record.args());
            }
        }
    }

    fn flush(&self) {}
}
