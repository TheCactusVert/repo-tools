use std::path::PathBuf;

use clap::{Arg, command, ArgMatches, Command};

pub enum Args {
    Clean(ArgsClean),
    Elephant(ArgsElephant),
    None,
}

pub struct ArgsClean {
    pub db_name: String,
    pub working_dir: PathBuf,
}

pub struct ArgsElephant {
    pub number: i32,
}

impl Args {
    fn matches() -> ArgMatches {
        command!()
            .about("List of tools to manage a repository")
            .subcommand(ArgsClean::matches())
            .subcommand(ArgsElephant::matches())
            .subcommand_required(true)
            .get_matches()
    }

    pub fn parse() -> Result<Self, std::process::ExitCode> {
        let matches = Self::matches();
        
        match matches.subcommand() {
            Some((ArgsClean::COMMAND, sub_matches)) => Ok(Args::Clean(ArgsClean::parse(sub_matches)?)),
            Some((ArgsElephant::COMMAND, sub_matches)) => Ok(Args::Elephant(ArgsElephant::parse(sub_matches)?)),
            _ => Ok(Args::None),
        }
    }
}

impl ArgsClean {
    const COMMAND: &'static str = "clean";
    const ARG_DB_NAME: &'static str = "db_name";
    const ARG_WORK_DIR: &'static str = "working_dir";

    fn matches() -> Command<'static> {
        Command::new(ArgsClean::COMMAND)
            .about("Clean a repository from unused packages")
            .arg(Arg::new(ArgsClean::ARG_DB_NAME).required(true).takes_value(true).help("Database name"))
            .arg(Arg::new(ArgsClean::ARG_WORK_DIR).long("directory").short('C').required(false).takes_value(true).help("Directory of the database"))
    }

    fn parse(matches: &ArgMatches) -> Result<Self, std::process::ExitCode> {
        Ok(Self {
            db_name: match matches.value_of(ArgsClean::ARG_DB_NAME) {
                Some(db) => db.to_string(),
                None => {
                    log::error!("Oops this shouldn't have happened, there is no name to the database.");
                    return Err(std::process::ExitCode::FAILURE);
                }
            },
            working_dir: match matches.value_of(ArgsClean::ARG_WORK_DIR) {
                Some(dir) => PathBuf::from(dir),
                None => std::env::current_dir().map_err(|e| {
                    log::error!("Couldn't get current directory: {}.", e.to_string());
                    std::process::ExitCode::FAILURE
                })?,
            },
        })
    }
}

impl ArgsElephant {
    const COMMAND: &'static str = "elephant";
    const ARG_ID: &'static str = "id";

    fn matches() -> Command<'static> {
        Command::new(ArgsElephant::COMMAND)
            .about("Toot toot")
            .arg(Arg::new(ArgsElephant::ARG_ID).long("id").required(false).takes_value(true))
    }
    
    fn parse(matches: &ArgMatches) -> Result<Self, std::process::ExitCode> {
        Ok(Self {
            number: matches.value_of_t::<i32>(ArgsElephant::ARG_ID).unwrap_or(0),
        })
    }
}
