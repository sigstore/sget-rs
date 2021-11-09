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

mod utils;
mod policy;

use clap::{App, Arg};
fn main() {
    let matches = App::new("sget")
        .version("0.1")
        .author("Sigstore Developers")
        .about("Secure script retrieval and execution")
        .license("Apache-2.0")
        .arg(
            Arg::new("oci-registry")
                .about("OCI registry namespace")
                .index(1),
        )
        .arg(
            Arg::new("noexec")
                .short('n')
                .long("noexec")
                .takes_value(false)
                .requires("oci-registry")
                .about("Do not execute script"),
        )
        .arg(
            Arg::new("outfile")
                .short('f')
                .long("outfile")
                .value_name("OUT_FILE")
                .requires("oci-registry")
                .about("Save script to file")
                .takes_value(true),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .takes_value(false)
                .conflicts_with("noexec")
                .about("Displays executing script's stdout to console"),
        )
        .get_matches();

    if let Some(o) = matches.value_of("oci-registry") {
        println!("OCI registry: {}", o);
    }
    if let Some(f) = matches.value_of("outfile") {
        println!("Output file: {}", f);
    }

    if !matches.is_present("noexec") {
        // TODO: When we can retrieve the blob, remove the below two lines
        // as these are temporary until we rig in the download / verify
        // functions
        let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("tests/test.sh");

        utils::run_script(&dir.to_string_lossy(), matches.is_present("interactive"))
            .expect("\n sget script execution failed");
        println!("\nsget script execution succeeded");
    }
}
