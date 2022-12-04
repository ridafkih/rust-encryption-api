use crate::handlers;
use actix_web::{web, App, HttpServer};
use std::env;

pub async fn start_server() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Failed to parse PORT");

    HttpServer::new(|| {
        App::new()
            .route(
                "/encrypt",
                web::post().to(handlers::crypto::handler_encrypt),
            )
            .route(
                "/decrypt",
                web::post().to(handlers::crypto::handler_decrypt),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
