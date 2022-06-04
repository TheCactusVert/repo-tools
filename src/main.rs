extern crate log;
extern crate tar;

mod args;
mod db;
mod logger;
mod package;

mod clean;
mod elephant;

use args::Args;

fn main() -> Result<(), std::process::ExitCode> {
    let args = Args::parse()?;

    logger::init().map_err(|e| {
        eprintln!("Couldn't load logger: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;
    
    match args {
        Args::Clean(args) => clean::execute(args),
        Args::Elephant(args) => elephant::execute(args),
        Args::None => Ok(()),
    }
}
