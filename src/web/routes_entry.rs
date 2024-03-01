use aws_sdk_dynamodb::Client;
use axum::{
    routing::{delete, get, post},
    Extension, Router,
};

use super::{create_entry, delete_entry, query_entry, scan_entries};

pub fn router(db_client: Client) -> Router {
    Router::new()
        .route("/", post(create_entry::create))
        .route("/:category/:created_at", delete(delete_entry::delete))
        .route("/:category/:created_at", get(query_entry::query))
        .route("/scan", get(scan_entries::scan))
        .layer(Extension(db_client))
}
