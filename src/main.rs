mod error;
mod event;
mod handler;
mod router;

use std::{env, sync::Arc};

use actix_web::{middleware::Logger, web, App, HttpServer};
use line_bot_sdk::Client;
use router::router;

pub struct AppState {
    line_client: Arc<Client>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(router)
            .app_data(web::Data::new(AppState {
                line_client: Arc::new(Client::new(
                    env::var("CHANNEL_ACCESS_TOKEN").unwrap(),
                    env::var("CHANNEL_SECRET").unwrap(),
                )),
            }))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
