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
    let lines = tokenizer::split_lines(body);
    let mut final_lines: Vec<Line> = vec![];
    return if let Some(filter) = q.filter {
        for mut line in lines {
            if (filter.starts_with(&line.name)) {
                let mut v = vec![];
                for field in line.sub {
                    if field.name.eq(&filter) {
                        v.push(field);
                    }
                }
                line.sub = v;
                final_lines.push(line);
            }
        }
        (StatusCode::default(), Json(final_lines))
    } else {
        (StatusCode::default(), Json(lines))
    }


}

#[derive(Serialize, Deserialize)]
struct Filter {
    pub filter: Option<String>
}
