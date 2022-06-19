use std::fs;
use std::io::prelude::*;

use crate::args;
use crate::db;
use crate::package::Package;

use anyhow::Result;
use globset::{Glob, GlobSetBuilder};
use tar::EntryType;

pub fn execute(args: args::SubcommandClean) -> Result<()> {
    // Open database
    let mut a = db::open(&args.database.directory, &args.database.name).map_err(|e| {
        log::error!("Couldn't open database: {}.", e.to_string());
        e
    })?;

    // List entries
    let entries = a.entries().map_err(|e| {
        log::error!("Couldn't read entries in database: {}.", e.to_string());
        e
    })?;

    // Create filter pattern
    let mut pattern: GlobSetBuilder = GlobSetBuilder::new();

    // Filter for database
    let db_pattern =
        Glob::new(&format!("*{}.{}*", args.database.name, "{db,files}")).map_err(|e| {
            log::error!("Glob: {}.", e.to_string());
            e
        })?;
    pattern.add(db_pattern);

    for file in entries {
        let mut file = match file {
            Ok(file) => file,
            Err(e) => {
                log::warn!("Couldn't open package description: {}.", e.to_string());
                continue;
            }
        };

        let header = file.header();
        if header.entry_type() != EntryType::Regular {
            continue;
        }

        let mut desc = String::new();
        if let Err(e) = file.read_to_string(&mut desc) {
            log::warn!("Couldn't read a package description: {}.", e.to_string());
            continue;
        };

        let pkg = Package::from_str(&desc);

        if pkg.filename.is_empty() {
            log::warn!("An entry in the database seems invalid.");
            continue;
        }

        log::info!("A package named {} has been found.", pkg.name);

        // Filter for package
        match Glob::new(&format!("*{}*", pkg.filename)) {
            Ok(package_pattern) => {
                pattern.add(package_pattern);
            }
            Err(e) => {
                log::warn!("Glob: {}.", e.to_string());
            }
        };
    }

    let pattern = pattern.build().map_err(|e| {
        log::error!("Glob: {}.", e.to_string());
        e
    })?;

    let paths_del = fs::read_dir(args.database.directory)
        .map_err(|e| {
            log::error!("Couldn't read database directory: {}.", e.to_string());
            e
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
