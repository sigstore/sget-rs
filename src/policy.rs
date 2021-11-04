use std::io::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::NonZeroU64;
use std::process::{Command, ExitStatus, Stdio};

// A signed root policy object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Policy {
    // A list of signatures.
    pub signatures: Vec<Signature>,
    // The root policy that is signed.
    pub signed: Root,
}

impl fmt::Display for Policy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

// A signature and the key ID and certificate that made it.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signature {
    // The hex encoded key ID that made this signature.
    pub keyid: String,
    // The base64 encoded signature of the canonical JSON of the root policy.
    pub sig: String,
    // The base64 encoded certificate that was used to create the signature.
    pub cert: String,
}

// The root policy indicated the trusted root keys.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "_type")]
pub struct Root {
    pub spec_version: String,
    pub version: NonZeroU64,
    pub namespace: String,
    pub expires: DateTime<Utc>,
    pub consistent_snapshot: bool,
    pub roles: HashMap<RoleType, RoleKeys>,
    pub keys: HashMap<String, Key>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RoleKeys {
    /// The key IDs used for the role.
    pub keyids: Vec<String>,
    /// The threshold of signatures required to validate the role.
    pub threshold: NonZeroU64,
}

/// The type of metadata role.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum RoleType {
    /// The root role delegates trust to specific keys trusted for all other top-level roles used in
    /// the system.
    Root,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "keytype")]
pub enum Key {
    /// A sigstore oidc key.
    SigstoreOidc {
        /// The sigstore oidc key.
        keyval: SigstoreOidcKey,
        /// Denotes the key's scheme
        scheme: String,
        /// Any additional fields read during deserialization; will not be used.
        // TODO: key_hash_algorithms
        #[serde(flatten)]
        _extra: HashMap<String, Value>,
    },
}

/// Represents a deserialized (decoded) SigstoreOidc public key.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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