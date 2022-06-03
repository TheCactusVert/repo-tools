extern crate tar;

mod package;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use package::Package;

use flate2::read::GzDecoder;
use tar::{Archive, EntryType};
use globset::{Glob, GlobSetBuilder};

fn open_databse(working_dir: &PathBuf, db_name: &str) -> Result<Archive<GzDecoder<BufReader<File>>>, std::process::ExitCode> {
    let db_path: PathBuf = working_dir.join(format!("{}.db.tar.gz", db_name));

    // Open file
    let db = File::open(db_path).map_err(|e| {
        log::error!("Couldn't open database: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;

    // Read file
    let buff = BufReader::new(db);

    // Decompress file
    let tar = GzDecoder::new(buff);

    // Open archive
    Ok(Archive::new(tar))
}

fn main() -> Result<(), std::process::ExitCode> {
    let db_name: String = String::from("cactus");
    let db_dir: Option<PathBuf> = None;

    pretty_env_logger::init();

    let working_dir = match db_dir {
        Some(dir) => dir,
        None => std::env::current_dir().map_err(|e| {
            log::error!("Couldn't get current directory: {}.", e.to_string());
            std::process::ExitCode::FAILURE
        })?,
    };
    
    // Open archive
    let mut a = open_databse(&working_dir, &db_name)?;

    // List entries
    let entries = a.entries().map_err(|e| {
        log::error!("Couldn't read entries in database: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;

    // Create filter pattern
    let mut pattern: GlobSetBuilder = GlobSetBuilder::new();

    // Filter for database
    let db_pattern = Glob::new(&format!("*{}.{}*", db_name, "{db,files}")).map_err(|e| {
        log::error!("Glob: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;
    pattern.add(db_pattern);

    for file in entries {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        let header = file.header();
        if header.entry_type() != EntryType::Regular {
            continue;
        }

        let mut desc = String::new();
        file.read_to_string(&mut desc).unwrap();

        let pkg = Package::from_str(&desc);

        if pkg.filename.is_empty() {
            log::warn!("An entry in the database seems invalid.");
        } else {
            log::info!("A package named {} has been found.", pkg.name);
            // Filter for package
            let package_pattern = Glob::new(&format!("*{}*", pkg.filename)).map_err(|e| {
                log::error!("Glob: {}.", e.to_string());
                std::process::ExitCode::FAILURE
            })?;
            pattern.add(package_pattern);
        }
    }

    let pattern = pattern.build().map_err(|e| {
        log::error!("Glob: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;

    let paths_del = fs::read_dir(working_dir)
        .map_err(|e| {
            log::error!("Couldn't read working directory: {}.", e.to_string());
            std::process::ExitCode::FAILURE
        })?
        .filter_map(|v| v.ok())
        .map(|v| v.path())
        .filter(|v| !pattern.is_match(v))
        .collect::<Vec<_>>();

    for path in paths_del {
        if let Err(e) = std::fs::remove_file(path) {
            log::warn!("Couldn't remove a file: {}.", e.to_string());
        }
    }

    Ok(())
}
