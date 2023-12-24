use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
/// Search for a pattern in a file and display the lines that contain it.
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let content: BufReader<File> = BufReader::new(file);

    for lines in content.lines() {
        let data: String = lines?;

        if data.contains(&args.pattern) {
            println!("{}", data);
        }
    }

    Ok(())
}
