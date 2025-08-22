use assert_cmd::Command;
#[test]
fn ok() {
    let mut cmd = Command::cargo_bin("ocfl-cli-rust").unwrap();
    cmd.assert().success();
}
