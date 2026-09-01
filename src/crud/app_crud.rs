use crate::models::app_models::AppInfo;
use crate::schema::app_info;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rocket::serde::json::serde_json;

// App info CRUD
pub fn get_app_data(conn: &mut SqliteConnection) -> QueryResult<AppInfo> {
    app_info::table.first::<AppInfo>(conn)
}

pub fn app_map(appinfo: &AppInfo) -> serde_json::Value {
    serde_json::json!({
        "app_id": appinfo.app_id,
        "app_name": appinfo.app_name,
        "app_semantic_version": appinfo.app_semantic_version,
        "app_named_version": appinfo.app_named_version,
        "homepage_welcome_banner": appinfo.homepage_welcome_banner,
        "homepage_welcome_text": appinfo.homepage_welcome_text,
        "about_us_text": appinfo.about_us_text,
        "goals_text": appinfo.goals_text,
    })
}
