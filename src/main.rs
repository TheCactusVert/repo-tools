extern crate log;
extern crate tar;

mod cli;
mod db;
mod logger;
mod package;
mod util;

mod commands;

use anyhow::Result;
use cli::{AppArgs, AppSubcommand};

fn main() -> Result<()> {
    /*sign::sign(std::path::PathBuf::from(
        "/tmp/toto/x86_64/cactus.db.tar.gz",
    ))?;*/

    let args = AppArgs::parse()?;

    logger::init(args.verbosity)?;

    match args.subcommand {
        AppSubcommand::Clean(args) => commands::clean::exec(args),
        AppSubcommand::Elephant(args) => commands::elephant::exec(args),
        AppSubcommand::None => Ok(()),
    }
}
