use actix_web::{get, error::ResponseError, web::Path, HttpResponse, web::Json};

#[get("/health_check")]
pub async fn health() -> Json<String> {
    return Json("Health Check Complete.".to_string());
}