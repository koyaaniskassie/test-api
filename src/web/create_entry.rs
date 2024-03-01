use aws_sdk_dynamodb::Client;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_dynamo::to_item;
use serde_json::{json, Value};

use crate::model::{Model, ModelFC};
use crate::{Result, TABLE_NAME};

pub async fn create(
    Extension(client): Extension<Client>,
    Json(payload): Json<ModelFC>,
) -> Result<(StatusCode, Json<Value>)> {
    let model = Model::new(payload);
    let item = to_item(model.clone())?;

    let req = client
        .put_item()
        .table_name(TABLE_NAME)
        .set_item(Some(item));

    req.send().await?;
    Ok((StatusCode::CREATED, Json(json!(model))))
}
