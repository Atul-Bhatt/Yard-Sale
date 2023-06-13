use crate::{
    repository::create_account::CreateAccountSchema,
    AppState
};

use actix_web::{ get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/health_check")]
async fn health() -> impl Responder {
    const MESSAGE: &str = "Yard Sale: Sell and buy products at a reasonable price.";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("/signup")]
async fn create_account(
    body: web::Json<CreateAccountSchema>,
    data: web::Data<AppState>
) -> impl Responder {
    let query_result = sqlx::query_as!(
        Account,
        "INSERT INTO accounts (email, password) VALUES ($1, $2)",
        body.email,
        &[body.password.to_owned()]
    )
    .fetch_one(&data.db)
    .await;

    HttpResponse::Ok()
}