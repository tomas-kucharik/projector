extern crate argparse;

use std::{
    ffi::OsStr,
    fs,
    io::Result,
    path::{Path, PathBuf},
};

use argparse::{ArgumentParser, Store};

/// Get all paths from a specified dir except ignored files
fn list_dir(dir: &PathBuf, ignored_files: &Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|entry| {
            let p = match entry {
                Ok(entry) => entry.path(),
                Err(error) => panic!("Problem listing the dir, {}", error),
            };

            let file_name = p.file_name().unwrap().to_str().unwrap();
            if file_name.starts_with('.') && !file_name.eq(".git"){
                // println!("found hidden {:?}", p.to_str().unwrap());

                return None;

            } else {
                return Some(p);
            }

            // if ignored_files
            //     .iter()
            //     .any(|ign| Some(OsStr::new(ign)) == p.file_name())
            // {
            //     None
            // } else {
            //     Some(p)
            // }
        })
        .collect();

    paths.sort();

    Ok(paths)
}

/// Get paths of only directories
fn list_dirs(dir: &PathBuf, ignored_files: &Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let paths = list_dir(dir, &ignored_files)?;

    let dirs: Vec<PathBuf> = paths.into_iter().filter(|path| path.is_dir()).collect();

    return Ok(dirs);
}

/// Checks whether the directory contains any basic files
fn does_dir_contain_files(dir: &PathBuf, ignored_files: &Vec<PathBuf>) -> Result<bool> {
    Ok(list_dir(&dir, &ignored_files)?
        .iter()
        .any(|path| path.is_file()))
}

/// Checks if dir is considered to be a project dir (has .git dir inside)
fn is_dir_project(dir: &PathBuf, ignored_files: &Vec<PathBuf>) -> Result<bool> {
    for path in list_dir(&dir, &ignored_files)? {
        if path.file_name() == Some(OsStr::new(".git")) {
            return Ok(true);
        }
    }

    return Ok(false);
}

/// Loads filenames from .projector_ignore file which will be ignored
fn get_ignored_files() -> Result<Vec<PathBuf>> {
    let files = fs::read_to_string("./src/.projector_ignore")?
        .lines()
        .map(|filename| PathBuf::from(filename))
        .collect();

    return Ok(files);
}

fn main() -> Result<()> {
    let mut root_dir: String = "./test_dir".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Tool which finds all subdirectories with .git/ directory inside");
        ap.refer(&mut root_dir)
            .add_option(&["-d", "--dir"], Store, "Path to the root directory");
        ap.parse_args_or_exit();
    }

    let ignored_files = get_ignored_files()?;
    let root_path = Path::new(&root_dir).to_path_buf();

    // println!("Finding projects inside {}", root_dir);

    let mut stack: Vec<PathBuf> = Vec::new();
    stack.push(root_path);

    let mut projects: Vec<PathBuf> = Vec::new();

    while let Some(path) = stack.pop() {
        // Check if dir is filled with only dirs
        // println!("Popped {:?} out of stack {:?}", path, stack);

        if is_dir_project(&path, &ignored_files)? {
            // println!("{:?} is a project", path);

            projects.push(path);
        } else if !does_dir_contain_files(&path, &ignored_files)? {
            // println!("{:?} is not a project", path);

            // Get paths of dirs
            let sub_paths = list_dir(&path, &ignored_files)?;
            // println!(
            //     "{:?} contains dirs: {:?}\nPushing them to stack",
            //     path, sub_paths
            // );

            // Fill stack
            for sub_path in sub_paths {
                stack.push(sub_path);
            }
        } else {
            let sub_paths = list_dir(&path, &ignored_files)?;
            // println!(
            //     "{:?} is not a project and contains files: {:?}",
            //     path, sub_paths
            // );
        }

        // println!();
    }

    println!("Found these projects: {:?}", projects);

    // let paths = list_dirs(&root_path)?;
    // // println!("{:?}", paths);

    // let ignored_files = get_ignored_files()?;

    // for path in paths {
    //     let contains_files = does_dir_contain_files(&path, &ignored_files);

    //     println!(
    //         "path: {:?} contains files? -> {:?}",
    //         path,
    //         contains_files.unwrap()
    //     );
    // }

    Ok(())
}
