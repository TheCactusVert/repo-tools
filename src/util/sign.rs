use std::ffi::OsString;
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use gpgme::context::Context;
use gpgme::Protocol;

fn append_sig_extension(path: &Path) -> PathBuf {
    let mut path: OsString = path.into();
    path.push(".sig");
    path.into()
}

pub fn sign(database: PathBuf) -> Result<()> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;

    let mut output = File::create(append_sig_extension(&database))?;
    let mut input = File::open(database)?;

    ctx.sign_detached(&mut input, &mut output)?;

    Ok(())
}
