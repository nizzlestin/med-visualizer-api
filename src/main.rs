mod tokenizer;
mod exporter;

use std::net::{IpAddr, SocketAddr};
use axum::{Json, Router};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::{post};
use serde::{Deserialize, Serialize};
use crate::tokenizer::Line;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/message", post(post_message));
    let addr = SocketAddr::from((IpAddr::from([127,0,0,1]), 8001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn post_message(
    Query(q): Query<Filter>,
    body: String
) -> (StatusCode, Json<Vec<Line>>) {
    let lines = tokenizer::split_lines(body, q);
    (StatusCode::default(), Json(lines))
}

#[derive(Serialize, Deserialize)]
struct Filter {
    pub segment_filter: Option<String>,
    pub field_filter: Option<String>
}
