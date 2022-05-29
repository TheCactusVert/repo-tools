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

fn main() -> Result<(), std::process::ExitCode> {
    let db_name: String = String::from("cactus");
    let db_dir: Option<PathBuf> = Some(PathBuf::from("/tmp/tmp.avLFTk0K8p")); 
    
    pretty_env_logger::init();

    let working_dir = match db_dir {
        Some(dir) => dir,
        None => std::env::current_dir().map_err(|e| {
            log::error!("Error getting current directory: {}.", e.to_string());
            std::process::ExitCode::FAILURE
        })?
    };

    let db_path: PathBuf = working_dir.join(format!("{}.db.tar.gz", db_name));

    // Open file
    let db = File::open(db_path).map_err(|e| {
        log::error!("Error reading database: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;

    // Read file
    let buff = BufReader::new(db);

    // Decompress file
    let tar = GzDecoder::new(buff);

    // Open archive
    let mut a = Archive::new(tar);

    // List entries
    let entries = a.entries().map_err(|e| {
        log::error!("Error reading database: {}.", e.to_string());
        std::process::ExitCode::FAILURE
    })?;
    
    let mut paths_keep = vec![
        working_dir.join(format!("{}.db", db_name)),
        working_dir.join(format!("{}.files", db_name)),
        working_dir.join(format!("{}.db.sig", db_name)),
        working_dir.join(format!("{}.files.sig", db_name)),
        working_dir.join(format!("{}.db.tar.gz", db_name)),
        working_dir.join(format!("{}.files.tar.gz", db_name)),
        working_dir.join(format!("{}.db.tar.gz.sig", db_name)),
        working_dir.join(format!("{}.files.tar.gz.sig", db_name)),
        working_dir.join(format!("{}.db.tar.gz.old", db_name)),
        working_dir.join(format!("{}.files.tar.gz.old", db_name)),
        working_dir.join(format!("{}.db.tar.gz.old.sig", db_name)),
        working_dir.join(format!("{}.files.tar.gz.old.sig", db_name))
    ];

    for file in entries {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        let header = file.header();
        if header.entry_type() != EntryType::Regular {
            continue;
        }

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        let pkg = Package::from_str(&s);
        
        if pk.filename.is_empty() {
            log::warn!("An entry in the database seems invalid.");
        } else {
            paths_keep.push(working_dir.join(format!("{}", pkg.filename)));
            paths_keep.push(working_dir.join(format!("{}.sig", pkg.filename)));
        }
    }
    
    
    let paths_del = fs::read_dir(working_dir)
        .map_err(|e| {
            log::error!("Error while reading directory: {}.", e.to_string());
            std::process::ExitCode::FAILURE
        })?
        .filter_map(|v| v.ok())
        .map(|v| v.path())
        .filter(|v| !paths_keep.contains(v))
        .collect::<Vec<_>>();

    for path in paths_del {
        if let Err(e) = std::fs::remove_file(path) {
             log::warn!("Error removing a file: {}.", e.to_string());
        }
    }
    
    log::info!("Hello");

    Ok(())
}
