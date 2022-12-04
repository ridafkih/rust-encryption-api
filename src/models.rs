use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EncryptRequest {
    pub plaintext: String,
}

#[derive(Deserialize, Serialize)]
pub struct EncryptResponse {
    pub ciphertext: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct DecryptRequest {
    pub ciphertext: String,
    pub key: String,
}

#[derive(Deserialize, Serialize)]
pub struct DecryptResponse {
    pub plaintext: String,
}
