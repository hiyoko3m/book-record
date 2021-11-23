use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSessionStorage {
    pub nonce: String,
    pub pkce_verifier: String,
}

impl LoginSessionStorage {
    pub fn new(nonce: &str, pkce_verifier: &str) -> Self {
        Self {
            nonce: nonce.to_string(),
            pkce_verifier: pkce_verifier.to_string(),
        }
    }
}
