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

mod oci;
pub mod policy;
mod utils;
use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::Write;

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

    // TODO: need better error handling in place of unwrap
    let reference = matches
        .value_of("oci-registry")
        .expect("image reference failed");

    let result = oci::blob_pull(reference).await;
    match result {
        Ok(data) => {
            if matches.is_present("exec") {
                // Write to tmpfile and execute it.
                let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                dir.push("tests/test.sh");

                let mut f = File::create(&dir).expect("Unable to create file");
                f.write_all(&data[..]).expect("Unable to write data");

                utils::run_script(&dir.to_string_lossy()).expect("\n sget execution failed");
                println!("\nsget execution succeeded");
            } else if matches.is_present("outfile") {
                let outfile = matches.value_of("outfile").unwrap(); //#[allow_ci]
                let cwd = env::current_dir().unwrap(); //#[allow_ci]
                let file = File::create(cwd.join(outfile));
                file.unwrap().write(&data).ok(); //#[allow_ci]
            } else {
                println!("{}", String::from_utf8(data).unwrap()); //#[allow_ci] // Print to stdout.
            }
        }
        Err(e) => {
            println!("File retrieval failed: {}", e);
        }
    }
}
