extern crate serde;
extern crate serde_json;
extern crate sodiumoxide;

use hex;
use std::str;

use sodiumoxide::crypto::secretbox;

use crate::models;

pub mod crypto {
    use super::{decrypt, encrypt};
    use crate::models;
    use actix_web::{web, HttpRequest, HttpResponse};

    pub async fn handler_encrypt(
        _request: HttpRequest,
        body: web::Json<models::EncryptRequest>,
    ) -> HttpResponse {
        let response = encrypt(body.plaintext.as_bytes());
        HttpResponse::Ok().json(response)
    }

    pub async fn handler_decrypt(
        _request: HttpRequest,
        body: web::Json<models::DecryptRequest>,
    ) -> HttpResponse {
        let result = decrypt(&body.ciphertext, &body.key);
        match result {
            Ok(plaintext) => HttpResponse::Ok().json(models::DecryptResponse { plaintext }),
            Err(error) => HttpResponse::BadRequest().body(error),
        }
    }
}

fn encrypt(plaintext: &[u8]) -> models::EncryptResponse {
    let nonce = secretbox::gen_nonce();
    let nonce_string = hex::encode(nonce);

    let key = secretbox::gen_key();
    let key_bytes = key.0;

    let ciphertext = secretbox::seal(plaintext, &nonce, &key);
    let ciphertext_hex = hex::encode(&ciphertext);

    models::EncryptResponse {
        ciphertext: ciphertext_hex + &nonce_string,
        key: hex::encode(key_bytes),
    }
}

fn decrypt(ciphertext: &str, key: &str) -> Result<String, &'static str> {
    let nonce_string = &ciphertext[ciphertext.len() - 48..];
    let ciphertext_string = &ciphertext[..ciphertext.len() - 48];

    let nonce_bytes =
        hex::decode(nonce_string).map_or(Err("Failed to decode nonce"), |bytes| Ok(bytes))?;
    let nonce = secretbox::Nonce::from_slice(nonce_bytes.as_slice())
        .map_or(Err("Failed to convert nonce bytes to Nonce"), |nonce| {
            Ok(nonce)
        })?;
    let ciphertext = hex::decode(ciphertext_string)
        .map_or(Err("Failed to decode ciphertext"), |bytes| Ok(bytes))?;

    let key_bytes = hex::decode(key).map_or(Err("Failed to decode key"), |bytes| Ok(bytes))?;
    let key = sodiumoxide::crypto::secretbox::Key::from_slice(&key_bytes)
        .map_or(Err("Failed to convert key to Key"), |key| Ok(key))?;

    secretbox::open(&ciphertext, &nonce, &key)
        .map_err(|_| "Failed to decrypt ciphertext")
        .map(|plaintext| {
            String::from_utf8(plaintext).expect("Failed to convert plaintext to string")
        })
}
