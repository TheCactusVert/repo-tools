use std::fs::File;

use std::io::BufReader;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use tar::Archive;

pub fn open(
    working_dir: &PathBuf,
    db_name: &str,
) -> Result<Archive<GzDecoder<BufReader<File>>>, std::io::Error> {
    let db_path: PathBuf = working_dir.join(format!("{}.db.tar.gz", db_name));

    // Open file
    let db = File::open(db_path)?;

    // Read file
    let buff = BufReader::new(db);

    // Decompress file
    let tar = GzDecoder::new(buff);

    // Open archive
    Ok(Archive::new(tar))
}
