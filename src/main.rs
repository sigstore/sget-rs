// Copyright 2021 The Sigstore Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use tempfile::tempdir;
mod oci;
pub mod policy;
mod utils;
use anyhow::Result;
use clap::{App, Arg};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::PermissionsExt;

// Example Usage: ./sget ghcr.io/jyotsna-penumaka/hello_sget:latest
// This will fetch the contents and print them to stdout.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("sget")
        .version("0.1")
        .author("Sigstore Developers")
        .about("Secure script retrieval and execution")
        .arg(
            Arg::new("oci-registry")
                .help("OCI registry namespace")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("exec")
                .long("exec")
                .takes_value(false)
                .help("Execute script"),
        )
        .arg(
            Arg::new("outfile")
                .short('f')
                .long("outfile")
                .value_name("OUT_FILE")
                .help("Save script to file")
                .takes_value(true),
        )
        .get_matches();

    let reference = matches.value_of("oci-registry").unwrap_or("");

    let data = oci::blob_pull(reference).await?;

    if matches.is_present("exec") {
        // Write contents to a tempfile, make it executable, and execute it.
        let dir = tempdir()?;
        let filepath = dir.path().join("sget-tmp.sh");

        let mut f = File::create(&filepath)?;
        let md = f.metadata()?;
        let mut perms = md.permissions();

        // Setting executable mode only on non-Windows.
        #[cfg(not(target_os = "windows"))]
        perms.set_mode(0o777); // Make the file executable.

        fs::set_permissions(&filepath, perms)?;

        f.write_all(&data[..])?;

        utils::run_script(&filepath.to_string_lossy()).expect("Execution failed");
        println!("Execution succeeded");
    } else if matches.is_present("outfile") {
        let outfile = matches.value_of("outfile").unwrap_or("");
        let cwd = env::current_dir()?;
        let mut file = File::create(cwd.join(outfile))?;
        file.write_all(&data)?;
    } else {
        let str = String::from_utf8(data)?;
        println!("{}", str); // Print to stdout.
    }
    anyhow::Ok(())
}
