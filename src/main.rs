mod api;
mod model;
mod repository;

use crate::api::home::health;

use actix_web::{HttpServer, App, web, middleware::Logger};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
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
        App::new()
        .app_data(web::Data::new(AppState {db: pool.clone() }))
        .wrap(logger)
        .service(health)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
