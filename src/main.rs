use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use grm;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Find all repositories in a given folder and save their status to a file.
    Search {
        /// Folder to search for repositories in
        folder: PathBuf,
        /// Turn off recursively searching sub-folders
        #[clap(short, long)]
        non_recursive: bool,
        /// CSV file to save found repositories and status to,
        /// defaults to 'repo_status.csv' in given folder.
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search {
            folder,
            non_recursive,
            output,
        } => {
            println!("Searching {}", folder.to_string_lossy());

            let mut count = 0;
            let repos = grm::find_repositories(&folder, !non_recursive, &mut count)?;
            println!("\nFound {} repositories", repos.len());

            let output = match output {
                Some(o) => o.to_owned(),
                None => folder.join("repo_status.csv"),
            };

            grm::write_status_csv(repos, &output)?;
            println!("Written: {}", output.to_string_lossy());
        }
    };

    Ok(())
}
