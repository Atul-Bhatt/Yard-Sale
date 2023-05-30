use crate::model::listing::Listing;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/health_check")]
async fn health() -> impl Responder {
    const MESSAGE: &str = "Yard Sale: Sell and buy products at a reasonable price.";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}
