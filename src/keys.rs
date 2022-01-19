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

use openssl::nid::Nid;
use openssl::{
    ec::EcGroup, ec::EcKey
};
use std::fs::File;
use std::io::Write;
use openssl::symm::Cipher;

// Sigstore relies on NIST P-256
// NIST P-256 is a Weierstrass curve specified in FIPS 186-4: Digital Signature Standard (DSS):
// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf
// Also known as prime256v1 (ANSI X9.62) and secp256r1 (SECG)
// openssl dgst -sha1 -sign sget.key file.txt > signature
// openssl dgst -sha1 -verify sget.pub -signature signature file.txt

pub fn generate_keys(passw: String) -> Result<(), Box<dyn std::error::Error>> {
    let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1)?;
    let private_key = EcKey::generate(&group)?;

    let private_key_pem;

    match passw.is_empty() {
        true => {
            private_key_pem = private_key.private_key_to_pem()?;
        }
        false => {
            private_key_pem = private_key.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passw.as_bytes())?;
        }
    }

    let public_key = private_key.public_key();
    let ec_pub_key = EcKey::from_public_key(&group, public_key)?;
    let public_key_pem = &ec_pub_key.public_key_to_pem()?;

    let mut privkey_file = File::create("sget.key")?;
    privkey_file.write_all(String::from_utf8(private_key_pem.clone())?.as_bytes())?;

    let mut pubkey_file = File::create("sget.pub")?;
    pubkey_file.write_all(String::from_utf8(public_key_pem.clone())?.as_bytes())?;

    Ok(())
}


#[test]
fn test_generate_keys() {
    let res = generate_keys(String::from("foo"));
    assert!(res.is_ok());
}
