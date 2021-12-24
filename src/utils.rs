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

pub fn password_prompt() -> Result<String, Error> {
    let pass1 = rpassword::read_password_from_tty(Some("Enter a password: "))?;
    let pass2 = rpassword::read_password_from_tty(Some("Re-enter password: "))?;
    if pass1 != pass2 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Passwords do not match",
        ));
    }
    Ok(pass1)
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

    let res = run_script(&dir.to_string_lossy(), false);
    assert!(res.unwrap().success()); //#[allow_ci]
}


// #[test]
// fn password_prompt_success() {
//     let res = password_prompt();
//     assert!(res.unwrap().len() > 0);
// }

// #[test]
// fn password_prompt_fail() {
//     let res = password_prompt();
//     assert!(res.is_err());
// }