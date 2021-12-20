use std::fs::File;
use std::io::Write;
use std::env;
use oci_distribution::{client, secrets::RegistryAuth, Client, Reference};

pub async fn blob_pull(reference: &str, file_name: &str) {
    let reference: Reference = reference.parse().expect("Invalid reference");
    let config = client::ClientConfig {
        protocol: client::ClientProtocol::Https,
        accept_invalid_hostnames: false,
        accept_invalid_certificates: false,
        extra_root_certificates: Vec::new(),
    };
    let mut client = Client::new(config);
    let auth: RegistryAuth = RegistryAuth::Anonymous;
    let accepted_media_types = vec!["text/plain"];
    let image = client
        .pull(&reference, &auth, accepted_media_types)
        .await
        .unwrap() //#[allow_ci]
        .layers
        .into_iter()
        .next()
        .map(|layer| layer.data);
    // print image variable
    match image {
        Some(image) => {
            let cwd = env::current_dir().unwrap(); //#[allow_ci]
            let file = File::create(cwd.join(file_name));
            file.unwrap().write_all(&image[..]).ok(); //#[allow_ci]
            println!("Success! Pulled the script!");
        }
        None => println!("Error!"),
    }
}