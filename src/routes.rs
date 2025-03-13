use std::collections::HashMap;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;

pub(crate) async fn pong_header(http_request: HttpRequest) -> impl Responder {
    let headers: HashMap<String, String> = http_request
        .headers()
        .iter()
        .map(|(key, value)| {
            (
                key.to_string(),
                value.to_str().unwrap_or("Invalid UTF-8").to_string(),
            )
        })
        .collect();

    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .json(json!({
            "headers": headers,
        }))
}
pub(crate) async fn pong_others(_http_request: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().finish()
}
