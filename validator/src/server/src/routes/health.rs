use super::Response;
use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}
