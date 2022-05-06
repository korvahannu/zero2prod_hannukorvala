//! lib.rs

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

// actix-web invokes from_request method and tries to deserialise the body
// into FormData. If it fails, it will automatically respond with 400 BAD REQUEST
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener : TcpListener) -> Result<Server, std::io::Error> {
    // HttpServer handles all transport-level concerns
    let server = HttpServer::new(|| {
        // App is where all your application logic lives
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
