mod exporter;
mod tokenizer;

use crate::tokenizer::{Field, Line};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let config = ServerConfig::parse();
    let serve_dir_from_dist = ServeDir::new("frontend/dist/");
    let app = Router::new()
        .nest_service("/", serve_dir_from_dist)
        .route("/message", post(post_message));
    let addr = SocketAddr::from((config.host, config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn post_message(Query(q): Query<Filter>, body: String) -> (StatusCode, Json<Vec<Vec<Field>>>) {
    let lines = tokenizer::split_lines(body, q);
    (StatusCode::default(), Json(lines))
}

#[derive(Serialize, Deserialize)]
struct Filter {
    pub segment_filter: Option<String>,
    pub field_filter: Option<String>,
}

#[derive(Debug, Parser)]
pub struct ServerConfig {
    #[clap(default_value = "127.0.0.1", env)]
    pub host: IpAddr,
    #[clap(default_value = "8060", env)]
    pub port: u16,
}
