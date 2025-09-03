use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args_builder() -> Result<()> {
    let mut cmd = Command::cargo_bin("echo-builder")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn dies_no_args_derive() -> Result<()> {
    let mut cmd = Command::cargo_bin("echo-derive")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

fn run(program: &str, args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(program)?
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn hello1_builder() -> Result<()> {
    run(
        "echo-builder",
        &["Hello there"],
        "tests/echo/expected/hello1.txt",
    )
}

#[test]
fn hello2_builder() -> Result<()> {
    run(
        "echo-builder",
        &["Hello", "there"],
        "tests/echo/expected/hello2.txt",
    )
}

#[test]
fn hello1_no_newline_builder() -> Result<()> {
    run(
        "echo-builder",
        &["Hello  there", "-n"],
        "tests/echo/expected/hello1.n.txt",
    )
}

#[test]
fn hello2_no_newline_builder() -> Result<()> {
    run(
        "echo-builder",
        &["-n", "Hello", "there"],
        "tests/echo/expected/hello2.n.txt",
    )
}

#[test]
fn hello1_derive() -> Result<()> {
    run(
        "echo-derive",
        &["Hello there"],
        "tests/echo/expected/hello1.txt",
    )
}

#[test]
fn hello2_derive() -> Result<()> {
    run(
        "echo-derive",
        &["Hello", "there"],
        "tests/echo/expected/hello2.txt",
    )
}

#[test]
fn hello1_no_newline_derive() -> Result<()> {
    run(
        "echo-derive",
        &["Hello  there", "-n"],
        "tests/echo/expected/hello1.n.txt",
    )
}

#[test]
fn hello2_no_newline_derive() -> Result<()> {
    run(
        "echo-derive",
        &["-n", "Hello", "there"],
        "tests/echo/expected/hello2.n.txt",
    )
}
