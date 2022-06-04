extern crate log;
extern crate tar;

mod args;
mod db;
mod logger;
mod package;

mod clean;

use args::Args;

fn main() -> Result<(), std::process::ExitCode> {

    let args = Args::parse()?;

    logger::init().map_err(|e| {
        eprintln!("Couldn't load logger: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;
    
    match args {
        Args::Clean(args) => clean::run(args),
        Args::None => Ok(()),
    }
}
