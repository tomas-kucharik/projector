use std::{ffi::OsStr, fs, io::Result, path::PathBuf};

/// Get all paths from a specified dir except hidden files
pub fn list_dir(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|entry| {
            let p = match entry {
                Ok(entry) => entry.path(),
                Err(error) => panic!("Problem listing the dir, {}", error),
            };

            let file_name = p.file_name().unwrap().to_str().unwrap();
            if file_name.starts_with('.') && !file_name.eq(".git") {
                return None;
            } else {
                return Some(p);
            }
        })
        .collect();

    paths.sort();

    Ok(paths)
}

/// Checks whether the directory contains any basic files
pub fn does_dir_contain_files(dir: &PathBuf) -> Result<bool> {
    Ok(list_dir(&dir)?.iter().any(|path| path.is_file()))
}

/// Checks if dir is considered to be a project dir (has .git dir inside)
pub fn is_dir_project(dir: &PathBuf) -> Result<bool> {
    for path in list_dir(&dir)? {
        if path.file_name() == Some(OsStr::new(".git")) {
            return Ok(true);
        }
    }

    return Ok(false);
}
