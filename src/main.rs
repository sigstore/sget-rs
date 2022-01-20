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
use std::path::Path;

// Example Usage: ./sget ghcr.io/jyotsna-penumaka/hello_sget:latest
// This will fetch the contents and print them to stdout.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = App::new("sget")
        .version("0.1")
        .author("Sigstore Developers")
        .about("Secure script retrieval and execution")
        .arg(
            Arg::new("ref")
                .help("OCI image reference")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("exec")
                .long("exec")
                .takes_value(false)
                .requires("outfile")
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
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .takes_value(false)
                .help("Displays executing script's stdout to console"),
        )
        .get_matches();

    let data = oci::blob_pull(matches.value_of("ref").unwrap_or("")).await?;

    if let Some(outfile) = matches.value_of("outfile") {
        let filepath = {
            let p = Path::new(outfile);
            if p.is_absolute() {
                p.into()
            } else {
                env::current_dir()?.join(outfile)
            }
        };

        let mut file = File::create(&filepath)?;
        file.write_all(&data[..])?;

        if matches.is_present("exec") {
            let md = file.metadata()?;
            let mut perms = md.permissions();
            // Setting executable mode only on non-Windows.
            #[cfg(not(target_os = "windows"))]
            perms.set_mode(0o777); // Make the file executable.
            fs::set_permissions(&filepath, perms)?;
            drop(file);

            utils::run_script(
                &filepath.to_string_lossy(),
                matches.is_present("interactive"),
            )
            .expect("Execution failed");
            eprintln!("\n\nExecution succeeded");
        }
    } else {
        println!("{}", String::from_utf8(data)?); // Print to stdout.
    }

    anyhow::Ok(())
}
