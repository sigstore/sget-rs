//
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
use clap::{App, Arg};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

// Example Usage: ./sget ghcr.io/jyotsna-penumaka/hello_sget:latest
// This will fetch the contents and print them to stdout.

#[tokio::main]
async fn main() {
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

    let reference = matches
        .value_of("oci-registry")
        .expect("OCI reference is required");

    let result = oci::blob_pull(reference).await;
    match result {
        Ok(data) => {
            if matches.is_present("exec") {
                // Write contents to a tempfile, make it executable, and execute it.
                let dir = tempdir().expect("Failed to create tempdir");
                let filepath = dir.path().join("sget-tmp.sh");

                let mut f = File::create(&filepath).expect("Failed to create file");
                let md = f.metadata().expect("Failed to get tempfile metadata");
                let mut perms = md.permissions();
                #[cfg(not(target_os = "windows"))] 
                perms.set_mode(0o777); // Make the file executable.
                fs::set_permissions(&filepath, perms)
                    .expect("Failed to set executable permissions");

                f.write_all(&data[..]).expect("Failed to write data");

                utils::run_script(&filepath.to_string_lossy()).expect("Execution failed");
                println!("Execution succeeded");
            } else if matches.is_present("outfile") {
                let outfile = matches.value_of("outfile").expect("outfile is required");
                let cwd = env::current_dir().expect("Failed to find current dir");
                let mut file = File::create(cwd.join(outfile)).expect("Failed to create file");
                file.write_all(&data).expect("Failed to write file");
            } else {
                let str = String::from_utf8(data).expect("Failed to interpret data as UTF-8");
                println!("{}", str); // Print to stdout.
            }
        }
        Err(e) => {
            println!("File retrieval failed: {}", e);
        }
    }
}
