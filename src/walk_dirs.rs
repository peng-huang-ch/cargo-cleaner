use crate::execute::clean;
use std::{fs, path};

pub fn walk_dirs(dir: &path::Path, depth: u32, max_depth: u32) -> eyre::Result<()> {
    if depth > max_depth {
        return Ok(());
    }
    // go through all files first to check if we're in a cargo root folder
    if dir.is_dir() {
        let mut entries = fs::read_dir(dir)?;

        // Run the command in the current directory should be a cargo root folder
        let _is_cargo_root = entries.find(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
                    if let Err(e) = clean(dir) {
                        eprintln!("Failed to clean directory {:?}: {:?}", dir, e);
                    }
                    return true;
                }
            }
            false
        });

        if depth == max_depth {
            return Ok(());
        }

        // Recurse into subdirectories
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    walk_dirs(&path, depth + 1, max_depth)?;
                }
            }
        }
    }
    Ok(())
}
