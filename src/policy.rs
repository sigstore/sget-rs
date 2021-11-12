use serde_json::Value;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_plain::{derive_display_from_serialize, derive_fromstr_from_deserialize};
use std::collections::HashMap;
use std::num::NonZeroU64;

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
#[derive(Serialize, Deserialize)]
pub struct Root {
    pub spec_version: String,
    pub version: NonZeroU64,
    pub namespace: String,
    pub expires: DateTime<Utc>,
    pub consistent_snapshot: bool,
    // TODO: better define RoleType, right now it doesn't match the actual data
    // The uncommended code will compile, but the unit test will fail because of the above
    //pub roles: HashMap<RoleType, RoleKeys>,
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
#[serde(tag = "keytype")]
pub enum Key {
    /// A sigstore oidc key.
    #[serde(rename = "sigstore-oidc")]
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

fn validate_expires(policy: Policy) -> chrono::Duration {
  let expiry = policy.signed.expires;
  let current = Utc::now();
  return expiry.signed_duration_since(current);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_script_success() {
        let mut fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        fixture.push("tests/test_data/policy_good.json");
        let raw_json = std::fs::read(fixture).expect("Cannot read test file");

        let policy: Policy = serde_json::from_slice(&raw_json).expect("Cannot deserialize Policy");
        assert_eq!(policy.signed.version, NonZeroU64::new(1).unwrap())
    }
  
    #[test]
    fn validate_expiry_success() {
        let mut fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        fixture.push("tests/test_data/policy_good.json");
        let raw_json = std::fs::read(fixture).expect("Cannot read test file");
        let policy: Policy = serde_json::from_slice(&raw_json).expect("Cannot deserialize Policy");
      
        let duration = validate_expires(policy);
        assert_eq!(duration.to_std().is_err(), false);
    }
  
    #[test]
    fn validate_expiry_failure() {
        let mut fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        fixture.push("tests/test_data/policy_bad.json");
        let raw_json = std::fs::read(fixture).expect("Cannot read test file");
        let policy: Policy = serde_json::from_slice(&raw_json).expect("Cannot deserialize Policy");
      
        let duration = validate_expires(policy);
        assert_eq!(duration.to_std().is_err(), true);
    }
}