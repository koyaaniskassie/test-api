use axum::{
    http::{Method, Uri},
    response::Response,
};
use serde::Serialize;
use serde_json::json;
use serde_with::skip_serializing_none;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn log_request(req_method: Method, uri: Uri, res: Response) -> Response {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let log_line = RequestLogLine {
        timestamp: timestamp.to_string(),
        req_path: uri.to_string(),
        req_method: req_method.to_string(),
    };

    println!("  ->> HTTP {}\n", json!(log_line));
    res
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    timestamp: String,
    req_path: String,
    req_method: String,
}
