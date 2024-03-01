use crate::error::Result;
use crate::TABLE_NAME;
use aws_sdk_dynamodb::types;
use aws_sdk_dynamodb::types::KeySchemaElement;
use aws_sdk_dynamodb::Client;
use std::process;

pub async fn create_table_if_not_exist(client: &Client, table: &str) {
    let exists = table_exists(client, table)
        .await
        .expect("->> Error checking if DynamoDB table exists");
    if !exists {
        println!(
            "->> DynamoDB table \"{}\" does not exist, creating...",
            TABLE_NAME
        );
        create_table(client).await;
    } else {
        println!(
            "->> DynamoDB table \"{}\" exists, skipping creation",
            TABLE_NAME
        );
    }
}

async fn table_exists(client: &Client, table: &str) -> Result<bool> {
    let table_list = client.list_tables().send().await;

    match table_list {
        Ok(list) => Ok(list.table_names().contains(&table.into())),
        Err(e) => Err(e.into()),
    }
}

async fn create_table(db_client: &Client) {
    let attr_partition_key = types::AttributeDefinition::builder()
        .attribute_name("category")
        .attribute_type(types::ScalarAttributeType::S)
        .build()
        .unwrap();

    let attr_sort_key = types::AttributeDefinition::builder()
        .attribute_name("created_at")
        .attribute_type(types::ScalarAttributeType::S)
        .build()
        .unwrap();

    let keyschema_pk = KeySchemaElement::builder()
        .attribute_name("category")
        .key_type(types::KeyType::Hash)
        .build()
        .unwrap();

    let keyschema_sk = KeySchemaElement::builder()
        .attribute_name("created_at")
        .key_type(types::KeyType::Range)
        .build()
        .unwrap();

    let create_result = db_client
        .create_table()
        .table_name(TABLE_NAME)
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .attribute_definitions(attr_partition_key)
        .attribute_definitions(attr_sort_key)
        .key_schema(keyschema_pk)
        .key_schema(keyschema_sk)
        .send()
        .await;

    if let Err(e) = create_result {
        println!("Creating DynamoDB table failed: {:#?}", e);
        process::exit(1);
    }
    println!("->> Table \"{}\" created!", TABLE_NAME);
}
