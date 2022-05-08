mod bot;
mod config;
mod router;

use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            // .wrap(verify_signature::Verifier)
            .wrap(Logger::default())
            .configure(router::router)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
