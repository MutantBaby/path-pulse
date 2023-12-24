use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use clap::Parser;

#[derive(Parser)]
/// Search for a pattern in a file and display the lines that contain it.
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let content = BufReader::new(File::open(&args.path).unwrap());

    for lines in content.lines() {
        let data: String = lines.unwrap();

        if data.contains(&args.pattern) {
            println!("{}", data);
        }
    }
}
