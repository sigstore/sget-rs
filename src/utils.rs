use std::io::Error;
use std::process::{Command, ExitStatus, Stdio};

pub(crate) fn run_script(path: &str) -> Result<ExitStatus, Error> {
    let mut childproc = Command::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Returns exit code of child process, or an error
    childproc.wait()
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

    let res = run_script(&dir.to_string_lossy());
    assert!(res.unwrap().success()); //#[allow_ci]
}
