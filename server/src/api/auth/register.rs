use crate::{
    repository::create_account::CreateAccountSchema,
    AppState
};

use actix_web::{ get, post, web, HttpResponse, Responder};
use serde_json::json;


#[get("/signup")]
async fn sign_up_page() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../resources/register.html"))
}


/// Register user using username and password
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

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("")
        .service(sign_up_page)
        .service(create_account));
}