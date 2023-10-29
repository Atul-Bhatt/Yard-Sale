use actix::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer};
use tokio::sync::broadcast;

use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::error::Error;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::accept_async;

use std::collections::HashMap;
use std::sync::Arc;

struct ChatServer {
    clients: Arc<tokio::sync::Mutex<HashMap<usize, actix::Addr<ChatClient>>>>,
    subscriber: broadcast::Sender<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = ChatServer::new().await;

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .route("/ws/", web::get().to(ws_index))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
