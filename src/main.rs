use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, Result};
use csv::Writer;

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

/// Git status of a repository
#[derive(Debug)]
struct Status {
    name: String,
    command_status: Option<i32>,
    branch: String,
    clean: bool,
    changes: String,
}

/// Checks and displays repository status.
fn repo_status(folder: &Path) -> Result<Status> {
    let output = Command::new("git")
        .args(["status", "-b", "--porcelain"])
        .current_dir(&folder)
        .output()?;

    let mut branch = String::new();
    let mut changes = String::new();

    let stdout = std::str::from_utf8(&output.stdout)?;

    for line in stdout.split('\n') {
        if line.starts_with('#') {
            branch.push_str(&format!("{}\n", line.trim_start_matches('#')));
        } else {
            changes.push_str(&format!("{}\n", line));
        }
    }

    branch = branch.trim().to_string();
    changes = changes.trim().to_string();
    let clean = changes.len() == 0;

    Ok(Status {
        name: match folder.file_name() {
            Some(s) => s.to_string_lossy().to_string(),
            None => "".to_string(),
        },
        command_status: output.status.code(),
        branch,
        clean,
        changes,
    })
}

/// Check status of repositories and write to CSV,
fn write_status_csv(repos: Vec<PathBuf>, file: &Path) -> Result<()> {
    let mut csv_file = Writer::from_path(file)?;

    csv_file.write_record([
        "path",
        "name",
        "command_status",
        "branch",
        "clean",
        "changes",
    ])?;

    for r in &repos {
        let status = repo_status(&r)?;

        csv_file.write_record([
            r.to_string_lossy().to_string(),
            status.name,
            match status.command_status {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
            status.branch,
            status.clean.to_string(),
            status.changes,
        ])?;
    }

    Ok(())
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
    println!("\nFound {} repositories\n{}", repos.len(), "-".repeat(100));

    let path = PathBuf::from("repo_status.csv");

    write_status_csv(repos, &path)?;

    println!("Written: {}", path.to_string_lossy());

    Ok(())
}
