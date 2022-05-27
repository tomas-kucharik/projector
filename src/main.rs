extern crate argparse;

use std::{
    io::Result,
    path::{Path, PathBuf},
};

use argparse::{ArgumentParser, Store};

mod utils;

fn main() -> Result<()> {
    let mut root_dir: String = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Blazingly fast tool, which walks the specified directory and finds all projects inside");
        ap.refer(&mut root_dir)
            .add_option(&["-d", "--dir"], Store, "Path to the directory you want to walk").required();
        ap.parse_args_or_exit();
    }

    let root_path = Path::new(&root_dir).to_path_buf();

    let mut stack: Vec<PathBuf> = Vec::new();
    stack.push(root_path);

    let mut projects: Vec<PathBuf> = Vec::new();

    while let Some(path) = stack.pop() {
        if utils::is_dir_project(&path)? {
            projects.push(path);
        } else if !utils::does_dir_contain_files(&path)? {
            // Get paths of dirs
            let sub_paths = utils::list_dir(&path)?;

            // Fill stack
            for sub_path in sub_paths {
                stack.push(sub_path);
            }
        }
    }

    projects.sort();

    projects.iter().for_each(|project| {
        println!("{}", project.to_str().unwrap());
    });

    Ok(())
}
