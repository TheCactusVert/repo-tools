use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::{command, Arg, ArgMatches, Command};
use openssl::rand::rand_bytes;

const ERROR_GENERIC: &'static str = "Required arguments were not provided";

pub struct Args {
    pub verbosity: Option<String>,
    pub subcommand: ArgsSubcommand,
}

pub enum ArgsSubcommand {
    Clean(ArgsClean),
    Elephant(ArgsElephant),
    None,
}

pub struct ArgsDatabase {
    pub db_name: String,
    pub working_dir: PathBuf,
}

pub struct ArgsClean {
    pub database: ArgsDatabase,
}

pub struct ArgsElephant {
    pub number: i32,
}

impl Args {
    const ARG_ID: &'static str = "verbosity";

    fn matches() -> ArgMatches {
        command!()
            .about("List of tools to manage a repository")
            .arg(
                Arg::new(Args::ARG_ID)
                    .long(Args::ARG_ID)
                    .short('v')
                    .env("RUST_LOG")
                    .required(false)
                    .takes_value(true)
                    .help("Verbosity level: error, warn, info, debug, trace"),
            )
            .subcommand(ArgsClean::command())
            .subcommand(ArgsElephant::command())
            .subcommand_required(true)
            .get_matches()
    }

    pub fn parse() -> Result<Self> {
        let matches = Self::matches();

        Ok(Args {
            verbosity: matches.value_of(Args::ARG_ID).map(str::to_string),
            subcommand: match matches.subcommand() {
                Some((ArgsClean::COMMAND, sub_matches)) => {
                    ArgsSubcommand::Clean(ArgsClean::parse(sub_matches)?)
                }
                Some((ArgsElephant::COMMAND, sub_matches)) => {
                    ArgsSubcommand::Elephant(ArgsElephant::parse(sub_matches)?)
                }
                _ => ArgsSubcommand::None,
            },
        })
    }
}

impl ArgsDatabase {
    const ARG_DB_NAME: &'static str = "db_name";
    const ARG_WORK_DIR: &'static str = "directory";

    fn get_args() -> Vec<Arg<'static>> {
        vec![
            Arg::new(ArgsDatabase::ARG_DB_NAME)
                .required(true)
                .takes_value(true)
                .help("Database name"),
            Arg::new(ArgsDatabase::ARG_WORK_DIR)
                .long(ArgsDatabase::ARG_WORK_DIR)
                .short('C')
                .required(false)
                .takes_value(true)
                .help("Directory of the database"),
        ]
    }

    fn parse(matches: &ArgMatches) -> Result<Self> {
        Ok(Self {
            db_name: matches
                .value_of(ArgsDatabase::ARG_DB_NAME)
                .ok_or(Error::msg(ERROR_GENERIC))? // Impossible but anyway
                .to_string(),
            working_dir: match matches.value_of(ArgsDatabase::ARG_WORK_DIR) {
                Some(dir) => PathBuf::from(dir),
                None => std::env::current_dir()?,
            },
        })
    }
}

impl ArgsClean {
    const COMMAND: &'static str = "clean";

    fn command() -> Command<'static> {
        Command::new(ArgsClean::COMMAND)
            .about("Clean a repository from unused packages")
            .args(ArgsDatabase::get_args())
    }

    fn parse(matches: &ArgMatches) -> Result<Self> {
        Ok(Self {
            database: ArgsDatabase::parse(matches)?,
        })
    }
}

impl ArgsElephant {
    const COMMAND: &'static str = "elephant";

    fn command() -> Command<'static> {
        Command::new(ArgsElephant::COMMAND).hide(true)
    }

    fn parse(_matches: &ArgMatches) -> Result<Self> {
        let mut buf = [0; 4];
        rand_bytes(&mut buf)?; // Using openssl, avoid adding dependencies

        Ok(Self {
            number: i32::from_ne_bytes(buf) % 3,
        })
    }
}
