mod bot;
mod config;
mod error;
mod models;
mod router;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(router::router))
        .bind(("localhost", 8080))?
        .run()
        .await
}
