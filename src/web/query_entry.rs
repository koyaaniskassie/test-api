use aws_sdk_dynamodb::{types::AttributeValue, Client};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_dynamo::aws_sdk_dynamodb_1::from_items;
use serde_json::{json, Value};

use crate::model::Model;
use crate::{Result, TABLE_NAME};

pub async fn query(
    Extension(db_client): Extension<Client>,
    Path((category, created_at)): Path<(String, String)>,
) -> Result<(StatusCode, Json<Value>)> {
    let query = db_client
        .query()
        .table_name(TABLE_NAME)
        .key_condition_expression("category = :category AND created_at >= :created_at")
        .expression_attribute_values(":category", AttributeValue::S(category))
        .expression_attribute_values(":created_at", AttributeValue::S(created_at));

    let res = query.send().await?;

    if let Some(items) = res.items {
        let tasks: Vec<Model> = from_items(items)?;
        return Ok((StatusCode::OK, Json(json!(tasks))));
    }

    panic!("{:?}", res.items);
}
