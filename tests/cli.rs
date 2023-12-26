use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::{error::Error, process::Command}; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd: Command = Command::cargo_bin("path_pulse")?;

    cmd.arg("foobar").arg("tests/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn empty_string_pattern() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("path_pulse")?;

    cmd.arg("").arg("tests/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Pattern must not be empty"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    let file: assert_fs::NamedTempFile = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd: Command = Command::cargo_bin("path_pulse")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}
