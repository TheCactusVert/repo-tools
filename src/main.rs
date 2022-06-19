extern crate log;
extern crate tar;

mod args;
mod db;
mod logger;
mod package;
mod sign;

mod clean;
mod elephant;

use anyhow::Result;
use args::{AppArgs, AppSubcommand};

fn main() -> Result<()> {
    /*sign::sign(std::path::PathBuf::from(
        "/tmp/toto/x86_64/cactus.db.tar.gz",
    ))?;*/

    let args = AppArgs::parse()?;

    logger::init(args.verbosity)?;

    match args.subcommand {
        AppSubcommand::Clean(args) => clean::execute(args),
        AppSubcommand::Elephant(args) => elephant::execute(args),
        AppSubcommand::None => Ok(()),
    }
}
