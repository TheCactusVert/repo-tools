use std::fmt::Arguments;
use std::io;
use std::io::Write;
use std::str::FromStr;

use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub fn init(max_level: Option<String>) -> Result<(), SetLoggerError> {
    let logger = Logger::default();

    log::set_max_level(match max_level {
        Some(max_level) => LevelFilter::from_str(&max_level).unwrap_or(LevelFilter::Warn),
        None => LevelFilter::Warn,
    });

    log::set_boxed_logger(Box::new(logger))?;
    Ok(())
}

#[derive(Default)]
struct Logger();

impl Logger {
    fn print(level: Level, args: &Arguments) -> io::Result<()> {
        let (bufwtr, color, text_level) = match level {
            Level::Error => (
                BufferWriter::stderr(ColorChoice::Always),
                Some(Color::Red),
                "error:",
            ),
            Level::Warn => (
                BufferWriter::stderr(ColorChoice::Always),
                Some(Color::Yellow),
                "warn:",
            ),
            Level::Info => (BufferWriter::stdout(ColorChoice::Always), None, "info:"),
            Level::Debug => (
                BufferWriter::stdout(ColorChoice::Always),
                Some(Color::Cyan),
                "debug:",
            ),
            Level::Trace => (
                BufferWriter::stdout(ColorChoice::Always),
                Some(Color::Magenta),
                "trace:",
            ),
        };

        let mut buffer = bufwtr.buffer();
        buffer.set_color(&ColorSpec::new().set_fg(color).set_bold(true))?;
        buffer.write(text_level.as_bytes())?;
        buffer.reset()?;
        writeln!(&mut buffer, " {}", args)?;
        bufwtr.print(&buffer)?;

        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        &metadata.level().to_level_filter() <= &log::max_level()
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
