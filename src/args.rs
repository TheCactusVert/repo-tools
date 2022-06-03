use clap::{arg, command};

pub struct Args {
    pub db_name: String,
}

impl Args {
    pub fn parse() -> Self {
        let matches = command!()
            .arg(arg!(<DB>).help("Database name"))
            .get_matches();

        Self {
            db_name: matches.value_of("DB").unwrap().to_string(),
        }
    }
}
