mod tokenizer;
mod exporter;

use std::net::{IpAddr, SocketAddr};
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello));
    let addr = SocketAddr::from((IpAddr::from([127,0,0,1]), 8001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn hello () -> &'static str {
    "hello"
}