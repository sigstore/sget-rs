use serde_json::{Value};
use serde::{Serialize,Deserialize};
use std::num::NonZeroU64;
use std::collections::HashMap;
use serde_plain::{derive_display_from_serialize, derive_fromstr_from_deserialize};
use chrono::{DateTime, Utc};
use structopt::StructOpt;

// A signed root policy object
#[derive(Serialize, Deserialize)]
pub struct Policy {
    // A list of signatures.
    pub signatures: Vec<Signature>,
    // The root policy that is signed.
    pub signed: Root,
}

// A signature and the key ID and certificate that made it.
#[derive(Serialize, Deserialize)]
pub struct Signature {
    // The hex encoded key ID that made this signature.
    pub keyid: String,
    // The base64 encoded signature of the canonical JSON of the root policy.
    pub sig: String,
    // The base64 encoded certificate that was used to create the signature.
    pub cert: String,
}

// The root policy indicated the trusted root keys.
#[derive(Serialize, Deserialize, StructOpt)]
pub struct Root {
    pub spec_version: String,
    pub version: NonZeroU64,
    pub namespace: String,
    pub expires: DateTime<Utc>,
    #[structopt(parse(try_from_str))]
    pub consistent_snapshot: bool,
    pub roles: HashMap<RoleType, RoleKeys>,
    pub keys: HashMap<String, Key>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleKeys {
    /// The key IDs used for the role.
    pub keyids: Vec<String>,
    /// The threshold of signatures required to validate the role.
    pub threshold: NonZeroU64,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
/// The type of metadata role.
pub enum RoleType {
    /// The root role delegates trust to specific keys trusted for all other top-level roles used in
    /// the system.
    Root,
}

derive_display_from_serialize!(RoleType);
derive_fromstr_from_deserialize!(RoleType);

#[derive(Serialize, Deserialize)]
pub enum Key {
    /// A sigstore oidc key.
    SigstoreOidc {
        /// The sigstore oidc key.
        keyval: SigstoreOidcKey,
        /// Denotes the key's scheme
        scheme: String,
        /// Any additional fields read during deserialization; will not be used.
        // TODO: key_hash_algorithms
        _extra: HashMap<String, Value>,
    },
}

derive_display_from_serialize!(Key);
derive_fromstr_from_deserialize!(Key);

#[derive(Serialize, Deserialize)]
/// Represents a deserialized (decoded) SigstoreOidc public key.
pub struct SigstoreOidcKey {
    /// The identity (subject)
    pub identity: String,
    /// The issuer
    pub issuer: String,
}

#[test]
fn parse_script_success() {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("tests/test_data/policy_good.json");

    let policy: Policy = serde_json::from_str(&json_string).unwrap();
}