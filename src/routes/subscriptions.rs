use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// actix-web invokes from_request method and tries to deserialise the body
// into FormData. If it fails, it will automatically respond with 400 BAD REQUEST
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
