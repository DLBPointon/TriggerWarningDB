use diesel::prelude::*;
use serde::Deserialize;

// app_info Models
#[derive(Queryable, Selectable, Deserialize)]
#[diesel(table_name = crate::schema::app_info)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AppInfo {
    pub app_id: Option<i32>,
    pub app_name: String,
    pub app_semantic_version: String,
    pub app_named_version: String,
}
