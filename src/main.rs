extern crate log;
extern crate tar;

mod args;
mod db;
mod logger;
mod package;

mod clean;
mod elephant;

use std::str::FromStr;

use anyhow::Result;
use args::Args;

fn main() -> Result<()> {
    let args = Args::parse()?;

    logger::init(
        log::LevelFilter::from_str(&std::env::var("RUST_LOG").unwrap_or("warn".to_string()))
            .unwrap_or(log::LevelFilter::Warn),
    )?; // TODO parse from args or env ?

    match args {
        Args::Clean(args) => clean::execute(args),
        Args::Elephant(args) => elephant::execute(args),
        Args::None => Ok(()),
    }
}
