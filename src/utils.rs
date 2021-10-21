extern crate execute;

// use std::path::PathBuf;
use std::process::Command;
use execute::Execute;
use std::io::Error;

pub(crate) fn run_script(path: &str) -> Result<Option<i32>, Error> {
    let mut command = Command::new(path);

    // TODO: we can feed in args for the script by using the following
    // command.arg("some-flag");

    let output = command.execute_output()?;

    Ok(output.status.code())
}


#[test]
fn execute_script_fail() {
    match run_script("i_don't_exist.txt") {
        Ok(_) => {}
        Err(err) => {
            assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
        }
    };
}


#[test]
fn execute_script_success() {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/test.sh");
    match run_script(&dir.to_string_lossy()) {
        Ok(result) => {
            assert_eq!(result, Some(0));
        }
        Err(err) => eprintln!("Error! {}", err)
    };
}