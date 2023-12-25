use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    path::PathBuf,
};

#[derive(Parser)]
/// Search for a pattern in a file and display the lines that contain it.
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

fn find_match(path: &PathBuf, pattern: &str, mut writer: impl Write) -> Result<()> {
    let file: File =
        File::open(path).with_context(|| format!("could not read file `{}`", path.display()))?;
    let content: BufReader<File> = BufReader::new(file);

    for lines in content.lines() {
        let data: String = lines?;

        if data.contains(pattern) {
            writeln!(writer, "{}", data)?;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    find_match(&args.path, &args.pattern, &mut stdout())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_find_match() {
        let path: PathBuf = PathBuf::from("./test.txt");
        let pattern: &str = "lorem";
        let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        let result: std::prelude::v1::Result<(), anyhow::Error> =
            find_match(&path, pattern, &mut writer);
        assert!(result.is_ok(), "Expected Ok(_), got {:?}", result);

        let output: Vec<u8> = writer.into_inner();
        let output_str: String = String::from_utf8(output).unwrap();

        assert!(
            output_str.contains(pattern),
            "Expected {} in output, got '{}'",
            pattern,
            output_str
        );
    }
}
