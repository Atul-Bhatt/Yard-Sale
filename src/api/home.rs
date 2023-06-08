use crate::{
    model::{
        listing::Listing,
        account::Account,
    },
    repository::create_account::CreateAccountSchema,
    AppState
};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
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
        "INSERT INTO accounts (email,password) VALUES ($1, $2)",
        body.email.to_string(),
        body.password.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(account) => {
            let account_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "account": account
            })});
            return HttpResponse::Ok().json(account_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail", "message": "Account with that email already exists."}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}