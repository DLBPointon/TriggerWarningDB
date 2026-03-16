use crate::schema::event_categories::dsl as ec;
use crate::schema::genres::dsl as ge;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::Serialize;

use crate::DbConn;

#[derive(Serialize)]
pub struct EventCategorys {
    id: i32,
    name: String,
}

#[get("/api/event_categories")]
pub async fn list_event_categories(conn: DbConn) -> Json<Vec<EventCategorys>> {
    let rows = conn
        .run(|c| {
            ec::event_categories
                .order(ec::id.asc())
                .select((ec::id, ec::name))
                .load::<(i32, String)>(c)
        })
        .await
        .unwrap_or_default();

    Json(
        rows.into_iter()
            .map(|(id, name)| EventCategorys { id, name })
            .collect(),
    )
}

#[get("/api/genres")]
pub async fn list_genre_categories(conn: DbConn) -> Json<Vec<EventCategorys>> {
    let rows = conn
        .run(|c| {
            ge::genres
                .order(ge::id.asc())
                .select((ge::id, ge::name))
                .load::<(i32, String)>(c)
        })
        .await
        .unwrap_or_default();

    Json(
        rows.into_iter()
            .map(|(id, name)| EventCategorys { id, name })
            .collect(),
    )
}
