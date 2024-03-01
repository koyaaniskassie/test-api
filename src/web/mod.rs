pub mod create_entry;
pub mod delete_entry;
pub mod query_entry;
pub mod routes_entry;
pub mod scan_entries;

pub const TABLE_NAME: &str = "test-api-table";
pub const MAX_SCAN_PAGE_SIZE: i32 = 100;
