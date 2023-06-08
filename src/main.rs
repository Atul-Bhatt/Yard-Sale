mod api;
mod model;
mod repository;
mod util;

use crate::api::home::{health, create_account};
use crate::util::auth;

use actix_web::{HttpServer, App, web, middleware::Logger, dev::ServiceRequest};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);

    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        {
            Ok(pool) => {
                println!("Connection to the database is successful!");
                pool
            },
            Err(err) => {
                println!("Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

        println!("Server started successfully!");

    HttpServer::new(move || {
        let logger = Logger::default();
        let auth = HttpAuthentication::bearer(validator);
        App::new()
        .app_data(web::Data::new(AppState {db: pool.clone() }))
        .wrap(auth)
        .wrap(logger)
        .service(health)
        .service(create_account)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
