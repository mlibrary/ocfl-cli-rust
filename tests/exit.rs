// use anyhow::Result;
use assert_cmd::Command;
// use predicates::prelude::*;
// use pretty_assertions::assert_eq;
// use std::fs;

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
