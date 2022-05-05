//! lib.rs

use actix_web::{web, App, HttpServer, HttpResponse, dev::Server};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    // HttpServer handles all transport-level concerns
    let server = HttpServer::new(|| {

        // App is where all your application logic lives
        App::new()
            .route("/health_check", web::get().to(health_check))

    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}