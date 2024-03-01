use aws_sdk_dynamodb::Client;
use axum::routing::get;
use axum::{middleware, Router};

mod db;
mod error;
mod log;
mod model;
mod web;

use db::create_table_if_not_exist;
use web::routes_entry;

pub use error::{AppError, Result};
pub use web::TABLE_NAME;

use crate::log::log_request;

#[tokio::main]
async fn main() -> Result<()> {
    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let sdk_config = aws_config::load_from_env().await;
    let db_client = Client::new(&sdk_config);

    create_table_if_not_exist(&db_client, TABLE_NAME).await;

    let all_routes = Router::new()
        .route("/", get(|| async {}))
        .nest("/api/v1/entry", routes_entry::router(db_client))
        .layer(middleware::map_response(log_request));

    let tcp_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    println!("Server is running on PORT={port}");
    axum::serve(tcp_listener, all_routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
