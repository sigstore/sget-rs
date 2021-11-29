use anyhow::{anyhow, Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_plain::{derive_display_from_serialize, derive_fromstr_from_deserialize};
use std::{collections::HashMap, convert::TryFrom, num::NonZeroU64};

// A signed root policy object
#[derive(Serialize, Deserialize)]
pub struct Policy {
    // A list of signatures.
    pub signatures: Vec<Signature>,
    // The root policy that is signed.
    pub signed: Signed,
}

impl Policy {
    pub fn validate_expires(&self) -> chrono::Duration {
        self.signed.expires.signed_duration_since(Utc::now())
    }
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
pub struct Signed {
    pub consistent_snapshot: bool,
    pub expires: DateTime<Utc>,
    pub keys: HashMap<String, Key>,
    pub namespace: String,
    pub roles: HashMap<String, RoleKeys>,
    pub spec_version: String,
    pub version: NonZeroU64,
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

impl TryFrom<&str> for RoleType {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        match s {
            "Root" => Ok(RoleType::Root),
            other => Err(anyhow!("Unknown RoleType: {}", other)),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::read,
        path::{Path, PathBuf},
    };

    const CRATE: &str = env!("CARGO_MANIFEST_DIR");

    struct Setup {
        good_policy: PathBuf,
        bad_policy: PathBuf,
    }

    impl Setup {
        fn new() -> Self {
            let good_policy = Path::new(CRATE).join("tests/test_data/policy_good.json");
            let bad_policy = Path::new(CRATE).join("tests/test_data/policy_bad.json");

            Self {
                good_policy,
                bad_policy,
            }
        }

        fn read_good_policy(&self) -> Policy {
            let raw_json = read(&self.good_policy).expect("Cannot read good policy file");
            serde_json::from_slice(&raw_json).expect("Cannot deserialize policy")
        }

        fn read_bad_policy(&self) -> Policy {
            let raw_json = read(&self.bad_policy).expect("Cannot read bad policy file");
            serde_json::from_slice(&raw_json).expect("Cannot deserialize policy")
        }
    }

    #[test]
    fn deserialize() {
        let setup = Setup::new();
        setup.read_good_policy();
    }

    #[test]
    fn parse_script_success() {
        let setup = Setup::new();
        let policy = setup.read_good_policy();
        assert_eq!(policy.signed.version, NonZeroU64::new(1).unwrap()) //#[allow_ci]
    }

    #[test]
    fn validate_expiry_success() {
        let setup = Setup::new();
        let policy = setup.read_good_policy();
        assert!(!policy.validate_expires().to_std().is_err());
    }

    #[test]
    fn validate_expiry_failure() {
        let setup = Setup::new();
        let policy = setup.read_bad_policy();
        assert!(policy.validate_expires().to_std().is_err());
    }
}
