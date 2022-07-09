use assert_cmd::Command;
use assert_fs::prelude::*;
use color_eyre::eyre::Result;
use predicates::prelude::*;

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
// run write to see if the command works
fn test_write() {
    let temp_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("garden").unwrap();
    let fake_editor_path = std::env::current_dir()
        .expect("expect to be in a dir")
        .join("tests")
        .join("fake-editor.sh");
    if !fake_editor_path.exists() {
        panic!("fake editor shell script not found");
    }
    let assert = cmd
        .env("GARDEN_PATH", temp_dir.path())
        .env("EDITOR", fake_editor_path.into_os_string())
        .arg("write")
        .arg("-t")
        .arg("atitle")
        .write_stdin("N\n".as_bytes())
        .assert();

    assert.success();

    temp_dir
        .child("atitle.md")
        .assert(predicate::path::exists());
}
