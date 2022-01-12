use std::io::Error;
use std::process::{Command, ExitStatus};

pub(crate) fn run_script(path: &str) -> Result<ExitStatus, Error> {
    // Returns exit code of child process, or an error.
    Command::new(path).spawn()?.wait()
}

#[test]
fn execute_script_fail() {
    assert_eq!(
        run_script("i_dont_exist.txt").unwrap_err().kind(),
        std::io::ErrorKind::NotFound
    );
}

#[test]
#[cfg(not(target_os = "windows"))]
fn execute_script_success() {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/test.sh");

    let res = run_script(&dir.to_string_lossy()).expect("Execution falied");
    assert!(res.success());
}
