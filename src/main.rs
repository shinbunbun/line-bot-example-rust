mod bot;
mod client;
mod config;
mod error;
mod models;
mod router;
mod verify_signature;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(router::router)
            .wrap(verify_signature::Verifier)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
