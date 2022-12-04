mod handlers;
mod models;
mod server;

extern crate serde;
extern crate serde_json;
extern crate sodiumoxide;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server::start_server().await
}
