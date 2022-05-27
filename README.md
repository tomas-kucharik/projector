# Projector
Blazingly fast tool, which walks through a specified directory and finds all projects

Directory is considered a project when it contains .git directory

## Installation
Simply clone this repo and cargo install from source (requires [rust](https://www.rust-lang.org/tools/install))
```bash
git clone git@github.com:tomas-kucharik/projector.git
cd projector
cargo install --path .
```

## Usage
```bash
Usage:
  projector [OPTIONS]

Blazingly fast tool, which walks the specified directory and finds all projects
inside

Optional arguments:
  -h,--help             Show this help message and exit
  -d,--dir DIR          Path to the directory you want to walk
```

### Examples
Walk `./test_dir` and find all projects
```bash
projector -d test_dir
```

Change directory with [fzf](https://github.com/junegunn/fzf)
```bash
cd $(projector -d ~/Documents | fzf)
```
