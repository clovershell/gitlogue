use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "git-logue",
    version = "0.1.0",
    about = "A Git history screensaver - watch your code rewrite itself",
    long_about = "git-logue is a terminal-based screensaver that replays Git commits as if a ghost developer were typing each change by hand. Characters appear, vanish, and transform with natural pacing and syntax highlighting."
)]
pub struct Args {
    #[arg(
        short,
        long,
        value_name = "PATH",
        help = "Path to Git repository (defaults to current directory)"
    )]
    pub path: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "HASH",
        help = "Replay a specific commit (single-commit mode)"
    )]
    pub commit: Option<String>,

    #[arg(
        short,
        long,
        value_name = "MS",
        default_value = "30",
        help = "Typing speed in milliseconds per character"
    )]
    pub speed: u64,
}

impl Args {
    pub fn validate(&self) -> Result<PathBuf> {
        let repo_path = self
            .path
            .clone()
            .unwrap_or_else(|| PathBuf::from("."));

        if !repo_path.exists() {
            anyhow::bail!("Path does not exist: {}", repo_path.display());
        }

        let git_dir = repo_path.join(".git");
        if !git_dir.exists() {
            anyhow::bail!(
                "Not a Git repository: {} (or any parent directories)",
                repo_path.display()
            );
        }

        repo_path
            .canonicalize()
            .context("Failed to resolve repository path")
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let repo_path = args.validate()?;

    println!("git-logue v0.1.0");
    println!("Repository: {}", repo_path.display());
    println!("Speed: {}ms per character", args.speed);

    if let Some(commit) = &args.commit {
        println!("Single-commit mode: {}", commit);
    } else {
        println!("Random commit loop mode");
    }

    Ok(())
}
