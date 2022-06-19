use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::{command, Arg, ArgMatches, Command};
use openssl::rand::rand_bytes;

const ERROR_GENERIC: &'static str = "Required arguments were not provided";

pub struct AppArgs {
    pub verbosity: Option<String>,
    pub subcommand: AppSubcommand,
}

pub enum AppSubcommand {
    Clean(SubcommandClean),
    Elephant(SubcommandElephant),
    None,
}

pub struct ArgsDatabase {
    pub name: String,
    pub directory: PathBuf,
}

pub struct SubcommandClean {
    pub database: ArgsDatabase,
}

pub struct SubcommandElephant {
    pub number: i32,
}

impl AppArgs {
    const ARG_ID: &'static str = "verbosity";

    fn matches() -> ArgMatches {
        command!()
            .about("List of tools to manage a repository")
            .arg(
                Arg::new(AppArgs::ARG_ID)
                    .long(AppArgs::ARG_ID)
                    .short('v')
                    .env("RUST_LOG")
                    .required(false)
                    .takes_value(true)
                    .help("Verbosity level: error, warn, info, debug, trace"),
            )
            .subcommand(SubcommandClean::get_command())
            .subcommand(SubcommandElephant::get_command())
            .subcommand_required(true)
            .get_matches()
    }

    pub fn parse() -> Result<Self> {
        let matches = Self::matches();

        Ok(AppArgs {
            verbosity: matches.value_of(AppArgs::ARG_ID).map(str::to_string),
            subcommand: match matches.subcommand() {
                Some((SubcommandClean::COMMAND, sub_matches)) => {
                    AppSubcommand::Clean(SubcommandClean::parse(sub_matches)?)
                }
                Some((SubcommandElephant::COMMAND, sub_matches)) => {
                    AppSubcommand::Elephant(SubcommandElephant::parse(sub_matches)?)
                }
                _ => AppSubcommand::None,
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
            name: matches
                .value_of(ArgsDatabase::ARG_DB_NAME)
                .ok_or(Error::msg(ERROR_GENERIC))? // Impossible but anyway
                .to_string(),
            directory: match matches.value_of(ArgsDatabase::ARG_WORK_DIR) {
                Some(dir) => PathBuf::from(dir),
                None => std::env::current_dir()?,
            },
        })
    }
}

impl SubcommandClean {
    const COMMAND: &'static str = "clean";

    fn get_command() -> Command<'static> {
        Command::new(SubcommandClean::COMMAND)
            .about("Clean a repository from unused packages")
            .args(ArgsDatabase::get_args())
    }

    fn parse(matches: &ArgMatches) -> Result<Self> {
        Ok(Self {
            database: ArgsDatabase::parse(matches)?,
        })
    }
}

impl SubcommandElephant {
    const COMMAND: &'static str = "elephant";

    fn get_command() -> Command<'static> {
        Command::new(SubcommandElephant::COMMAND).hide(true)
    }

    fn parse(_matches: &ArgMatches) -> Result<Self> {
        let mut buf = [0; 4];
        rand_bytes(&mut buf)?; // Using openssl, avoid adding dependencies

        Ok(Self {
            number: i32::from_ne_bytes(buf) % 3,
        })
    }
}
