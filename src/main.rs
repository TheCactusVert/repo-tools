extern crate log;
extern crate tar;

mod args;
mod db;
mod logger;
mod package;

use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use args::Args;
use package::Package;

use globset::{Glob, GlobSetBuilder};
use tar::EntryType;

fn main() -> Result<(), std::process::ExitCode> {
    let args = Args::parse();

    let db_dir: Option<PathBuf> = None;

    logger::init().map_err(|e| {
        eprintln!("Couldn't load logger: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;

    let working_dir = match db_dir {
        Some(dir) => dir,
        None => std::env::current_dir().map_err(|e| {
            log::error!("Couldn't get current directory: {}.", e.to_string());
            std::process::ExitCode::FAILURE
        })?,
    };

    // Open archive
    let mut a = db::open(&working_dir, &args.db_name).map_err(|e| {
        log::error!("Couldn't open database: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;

    // List entries
    let entries = a.entries().map_err(|e| {
        log::error!("Couldn't read entries in database: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;

    // Create filter pattern
    let mut pattern: GlobSetBuilder = GlobSetBuilder::new();

    // Filter for database
    let db_pattern = Glob::new(&format!("*{}.{}*", args.db_name, "{db,files}")).map_err(|e| {
        log::error!("Glob: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;
    pattern.add(db_pattern);

    for file in entries {
        let mut file = file.map_err(|e| {
            log::error!("Couldn't open package description: {}.", e.to_string());
            std::process::ExitCode::FAILURE
        })?;

        let header = file.header();
        if header.entry_type() != EntryType::Regular {
            continue;
        }

        let mut desc = String::new();
        file.read_to_string(&mut desc).map_err(|e| {
            log::error!("Couldn't read a package description: {}.", e.to_string());
            std::process::ExitCode::FAILURE
        })?;

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
        .filter(|v| !pattern.is_match(v));

    for path in paths_del {
        if let Err(e) = std::fs::remove_file(path) {
            log::warn!("Couldn't remove a file: {}.", e.to_string());
        }
    }

    Ok(())
}
