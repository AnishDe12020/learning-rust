use assert_cmd::Command;
use color_eyre::eyre::Result;

#[test]
// run help to see if the binary works
fn test_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden")?;
    let assert = cmd.arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
// run write with the help flag to see if the command exists
fn test_write_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden")?;
    let assert = cmd.arg("write").arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
#[ignore = "todo"]
// run write to see if the command works
fn test_write() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden").unwrap();
    let assert = cmd.arg("write").assert();
    assert.success().stderr("");
    Ok(())
}
