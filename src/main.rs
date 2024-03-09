mod exporter;
mod tokenizer;

use crate::tokenizer::Line;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() {
    let config = ServerConfig::parse();
    let app = Router::new().route("/message", post(post_message));
    let addr = SocketAddr::from((config.host, config.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn post_message(Query(q): Query<Filter>, body: String) -> (StatusCode, Json<Vec<Line>>) {
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
