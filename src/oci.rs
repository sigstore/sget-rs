use anyhow::Result;
use oci_distribution::{client, secrets::RegistryAuth, Client, Reference};

pub async fn blob_pull(reference: &str) -> Result<Vec<u8>, anyhow::Error> {
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
        .await?
        .layers
        .into_iter()
        .next()
        .map(|layer| layer.data);
    match image {
        Some(data) => Ok(data),
        None => Err(anyhow::anyhow!("Failed to fetch blob")), // TODO: Better error message.
    }
}