pub mod yaz0;
pub mod arc;

use arc::{Arc, DirectoryEntry};
use yaz0::Yaz0;

use std::path::Path;
use std::fs;

pub struct Szs {
    pub root: DirectoryEntry,
}

pub fn parse(path: &str) -> Result<Szs, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;

    let decompressed = Yaz0::decompress(&data)?;
    let archive = Arc::new(&decompressed.decompressed_data)?;

    Ok(Szs { root: archive.tree })
}

pub fn extract(path: &str, out: &str) -> Result<(), String> {
    let szs = parse(path)?;
    Arc::extract_dir(&szs.root, Path::new(out))?;
    Ok(())
}