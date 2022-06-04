extern crate log;
extern crate tar;

mod args;
mod db;
mod logger;
mod package;

mod clean;
mod elephant;

use args::Args;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse()?;

    logger::init()?;
    
    match args {
        Args::Clean(args) => clean::execute(args),
        Args::Elephant(args) => elephant::execute(args),
        Args::None => Ok(()),
    }
}
