extern crate serde;
extern crate serde_json;
extern crate sodiumoxide;

use hex;
use serde::Serialize;
use std::env;
use std::str;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;

use sodiumoxide::crypto::secretbox;

#[derive(Deserialize)]
struct EncryptRequest {
    plaintext: String,
}

#[derive(Deserialize, Serialize)]
struct EncryptResponse {
    ciphertext: String,
    key: String,
}

#[derive(Deserialize)]
struct DecryptRequest {
    ciphertext: String,
    key: String,
}

#[derive(Deserialize, Serialize)]
struct DecryptResponse {
    plaintext: String,
}

async fn encrypt_handler(_request: HttpRequest, body: web::Json<EncryptRequest>) -> HttpResponse {
    let plaintext = body.plaintext.as_bytes();
    let nonce = secretbox::gen_nonce();
    let nonce_string = hex::encode(nonce);
    let key = secretbox::gen_key();
    let key_bytes = key.0;
    let ciphertext = secretbox::seal(plaintext, &nonce, &key);
    let ciphertext_hex = hex::encode(&ciphertext);

    HttpResponse::Ok().json(EncryptResponse {
        ciphertext: ciphertext_hex + &nonce_string,
        key: hex::encode(key_bytes),
    })
}

async fn decrypt_handler(_request: HttpRequest, body: web::Json<DecryptRequest>) -> HttpResponse {
    let ciphertext_string = body.ciphertext.to_string();
    let nonce_iterator = ciphertext_string.chars();
    let ciphertext_iterator = ciphertext_string.chars();

    let nonce_string: String = nonce_iterator
        .skip(ciphertext_string.len() - 48)
        .take(48)
        .collect();

    let ciphertext_string: String = ciphertext_iterator
        .take(ciphertext_string.len() - 48)
        .collect();

    let nonce_bytes = hex::decode(&nonce_string).expect("Failed to decode nonce");

    let nonce = secretbox::Nonce::from_slice(nonce_bytes.as_slice()).unwrap();
    let ciphertext = hex::decode(&ciphertext_string).expect("Failed to decode ciphertext");

    let key = sodiumoxide::crypto::secretbox::Key::from_slice(
        &hex::decode(&body.key).expect("Failed to decode key"),
    )
    .expect("Failed to convert key to Key");

    let plaintext = match secretbox::open(&ciphertext, &nonce, &key) {
        Ok(plaintext) => plaintext,
        Err(_) => return HttpResponse::BadRequest().body("Failed to decrypt ciphertext"),
    };

    HttpResponse::Ok().json(DecryptResponse {
        plaintext: String::from_utf8(plaintext).expect("Failed to convert plaintext to string"),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Failed to parse PORT");

    HttpServer::new(|| {
        App::new()
            .route("/encrypt", web::post().to(encrypt_handler))
            .route("/decrypt", web::post().to(decrypt_handler))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
