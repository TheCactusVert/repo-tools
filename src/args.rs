use std::path::PathBuf;

use clap::{Arg, command, ArgMatches, Command};

pub enum Args {
    Clean(ArgsClean),
    None,
}

pub struct ArgsClean {
    pub db_name: String,
    pub working_dir: PathBuf,
}

impl Args {
    fn matches() -> ArgMatches {
        command!()
            .about("List of tools to manage a repository")
            .subcommand(ArgsClean::matches())
            .subcommand_required(true)
            .get_matches()
    }

    pub fn parse() -> Result<Self, std::process::ExitCode> {
        let matches = Self::matches();
        
        match matches.subcommand() {
            Some(("clean", sub_matches)) => Ok(Args::Clean(ArgsClean::parse(sub_matches)?)),
            _ => Ok(Args::None),
        }
    }
}

impl ArgsClean {
    fn matches() -> Command<'static> {
        Command::new("clean")
            .about("Clean a repository from unused packages")
            .arg(Arg::new("db_name").required(true).takes_value(true).help("Database name"))
            .arg(Arg::new("working_dir").long("directory").short('C').required(false).takes_value(true).help("Directory of the database"))
    }

    fn parse(matches: &ArgMatches) -> Result<Self, std::process::ExitCode> {
        Ok(Self {
            db_name: matches.value_of("db_name").unwrap().to_string(),
            working_dir: match matches.value_of("working_dir") {
                Some(dir) => PathBuf::from(dir),
                None => std::env::current_dir().map_err(|e| {
                    log::error!("Couldn't get current directory: {}.", e.to_string());
                    std::process::ExitCode::FAILURE
                })?,
            },
        })
    }
}
