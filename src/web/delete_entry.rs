use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Extension;

use crate::{Result, TABLE_NAME};

pub async fn delete(
    Extension(client): Extension<Client>,
    Path((category, created_at)): Path<(String, String)>,
) -> Result<StatusCode> {
    let req = client
        .delete_item()
        .table_name(TABLE_NAME)
        .key("category", AttributeValue::S(category))
        .key("created_at", AttributeValue::S(created_at));

    req.send().await?;
    Ok(StatusCode::NO_CONTENT)
}
