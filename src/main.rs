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
use args::{Args, ArgsSubcommand};

fn main() -> Result<()> {
    /*sign::sign(std::path::PathBuf::from(
        "/tmp/toto/x86_64/cactus.db.tar.gz",
    ))?;*/

    let args = Args::parse()?;

    logger::init(args.verbosity)?;

    match args.subcommand {
        ArgsSubcommand::Clean(args) => clean::execute(args),
        ArgsSubcommand::Elephant(args) => elephant::execute(args),
        ArgsSubcommand::None => Ok(()),
    }
}
