mod routes;

use crate::routes::{pong_header, pong_others};

use actix_web::{web, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Récupération et parsing du port
    let port: u16 = env::var("PING_LISTEN_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("Starting server on port: {}", port);

    HttpServer::new(|| {
        App::new().route("/ping", web::get().to(pong_header))
            .route("/*", web::get().to(pong_others))
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

