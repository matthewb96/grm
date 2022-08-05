use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

/// Expected name of the git folder
const GIT_DIR: &str = ".git";

/// Checks if given `folder` is a git repository
///
/// For this function a git repository is defined as a folder containing
/// a sub-folder named `GIT_DIR`.
fn is_repository(folder: &Path) -> Result<bool> {
    if !folder.is_dir() {
        return Ok(false);
    }

    for d in folder.read_dir()? {
        let d = d.unwrap();

        if d.file_name() == GIT_DIR && d.path().is_dir() {
            return Ok(true);
        };
    }

    return Ok(false);
}

/// Finds all git repositories within `folder`.
///
/// Displays a message for how many paths have been checked.
fn find_repositories(folder: &Path, recursive: bool, count: &mut usize) -> Result<Vec<PathBuf>> {
    print!("\rChecked {}", count);

    if !folder.is_dir() {
        return Err(anyhow!(
            "folder \"{}\" doesn't exist",
            folder.to_string_lossy()
        ));
    }

    let mut repos: Vec<PathBuf> = Vec::new();

    let dir_list = match folder.read_dir() {
        Ok(d) => d,
        Err(_) => return Ok(repos),
    };

    for path in dir_list {
        *count += 1;

        let path = match path {
            Ok(p) => p.path(),
            Err(_) => continue,
        };

        if !path.is_dir() {
            continue;
        }

        match is_repository(&path) {
            Ok(true) => {
                repos.push(path);
                continue;
            }
            Ok(false) => (),
            Err(_) => (),
        }

        if recursive {
            match find_repositories(&path, true, count) {
                Ok(mut r) => repos.append(&mut r),
                Err(_) => continue,
            };
        }
    }

    Ok(repos)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("expected argument for folder to check"));
    }

    let folder = PathBuf::from(&args[1]);

    println!("Searching {}", folder.to_string_lossy());

    let mut count = 0;
    let repos = find_repositories(&folder, true, &mut count)?;

    println!("\nFound {} repositories", repos.len());
    for r in repos {
        println!("{:?} is repo", r);
    }

    Ok(())
}
