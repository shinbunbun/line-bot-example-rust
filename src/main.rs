mod bot;
mod router;

use actix_web::{App, HttpServer};
// use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();

    HttpServer::new(move || App::new().configure(router::router))
        .bind(("localhost", 8080))?
        .run()
        .await
}
