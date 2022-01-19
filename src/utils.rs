use std::io::Error;
use std::process::{Command, ExitStatus, Stdio};

pub(crate) fn run_script(path: &str, interactive: bool) -> Result<ExitStatus, Error> {
    // TODO: we can feed in args for the script by using the following
    // command.arg("some-flag");
    let mut childproc = if interactive {
        Command::new(path).spawn()?
    } else {
        Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    };
    // Returns exit code of child process, or an error
    childproc.wait()
}

#[test]
fn execute_script_fail() {
    assert_eq!(
        run_script("i_dont_exist.txt", false).unwrap_err().kind(),
        std::io::ErrorKind::NotFound
    );
}

#[test]
#[cfg(not(target_os = "windows"))]
fn execute_script_success() {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/test.sh");

    let res = run_script(&dir.to_string_lossy(), false).expect("Execution falied");
    assert!(res.success());
}
