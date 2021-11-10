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
use oci_distribution::{Client,client,Reference,secrets::RegistryAuth};
use std::env;
use std::fs::File;
use std::io::Write;

async fn pull(reference: Reference, file_name: &str) {
    let config = client::ClientConfig {
        protocol: client::ClientProtocol::Https,
        accept_invalid_hostnames: false,
        accept_invalid_certificates: false,
        extra_root_certificates: Vec::new()
    };
    let mut client = Client::new(config);
    let auth: RegistryAuth = RegistryAuth::Anonymous;
    let accepted_media_types = vec!["text/plain"];
    let image = client.pull(&reference, &auth, accepted_media_types)
            .await
            .unwrap()
            .layers
            .into_iter()
            .next()
            .map(|layer| layer.data);
    match image {
        Some(image) => {
            let cwd = env::current_dir().unwrap();
            let file = File::create(cwd.join(file_name));
            file.unwrap().write_all(&image[..]).ok();
            println!("Success! Pulled the script!");
        }
        None => println!("Error!"),
    }
}

// Example Usage: ./sget --noexec --outfile file.sh ghcr.io/jyotsna-penumaka/hello_sget:latest

#[tokio::main]
async fn main() {
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

    // TO DO: need better error handling in place of unwrap
    let reference: Reference = matches.value_of("oci-registry").unwrap().parse().unwrap();
    let outfile = matches.value_of("outfile").unwrap();
    pull(reference,outfile).await;
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
