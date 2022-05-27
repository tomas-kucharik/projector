# Projector

Blazingly fast tool, which walks through a specified directory and finds all projects

Directory is considered a project when it contains .git directory


## Installation

Simply clone this repo and cargo install the binary
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
Basic usage
```bash
$ projector -d test_dir
test_dir/dir1/project1-1/sample
test_dir/project1
```
With fzf
```bash
$ projector -d test_dir | fzf
```
