use aws_sdk_dynamodb::Client;
use axum::{Extension, Json};
use serde_dynamo::aws_sdk_dynamodb_1::from_items;
use serde_json::{json, Value};

use crate::model::Model;
use crate::{Result, TABLE_NAME};

use super::MAX_SCAN_PAGE_SIZE;

pub async fn scan(Extension(db_client): Extension<Client>) -> Result<Json<Value>> {
    let items = db_client
        .scan()
        .table_name(TABLE_NAME)
        .limit(MAX_SCAN_PAGE_SIZE)
        .send()
        .await?
        .items;

    if let Some(items) = items {
        let tasks: Vec<Model> = from_items(items)?;
        println!("{:?}", tasks);
        return Ok(Json(json!(tasks)));
    }

    Ok(Json(json!([])))
}
