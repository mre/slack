use walkdir::WalkDir;
use std::fs::File;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use std::io::Read;

use errors::*;

fn get_content(filename: &Path) -> Result<String> {
    let mut contents = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_methods() -> HashMap<PathBuf, String> {
    let mut entries = HashMap::new();
    for entry in WalkDir::new("codegen/slack-api-ref/methods") {
        let entry = entry.unwrap();
        let content = get_content(entry.path());
        if content.is_ok() {
            entries.insert(entry.path().to_owned(), content.unwrap());
        }
    }
    entries
}
