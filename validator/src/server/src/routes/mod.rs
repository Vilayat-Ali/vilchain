pub mod health;

use actix_web::{HttpResponse, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

pub async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}
