use assert_cmd::Command;

#[test]
fn cli_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-cli-template")?;

    cmd.assert().success();
    Ok(())
}
