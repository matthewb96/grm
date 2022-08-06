use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};

use grm;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("expected argument for folder to check"));
    }

    let folder = PathBuf::from(&args[1]);

    println!("Searching {}", folder.to_string_lossy());

    let mut count = 0;
    let repos = grm::find_repositories(&folder, true, &mut count)?;
    println!("\nFound {} repositories\n{}", repos.len(), "-".repeat(100));

    let path = PathBuf::from("repo_status.csv");

    grm::write_status_csv(repos, &path)?;

    println!("Written: {}", path.to_string_lossy());

    Ok(())
}
