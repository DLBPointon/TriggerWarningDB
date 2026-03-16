use crate::DbConn;
use crate::auth::auth::AdminGuard;
use crate::models::movie_models::SeedEvent;
use rocket::serde::json::{Json, serde_json};

#[post("/movies/<id>/events", format = "json", data = "<event>")]
pub async fn api_add_event(
    admin: AdminGuard,
    id: i32,
    event: Json<SeedEvent>,
    conn: DbConn,
) -> Json<serde_json::Value> {
    use diesel::prelude::*;

    let result: Result<i32, diesel::result::Error> = conn
        .run(move |c| {
            use crate::schema::event_categories::dsl as ec;
            use crate::schema::movie_event_categories::dsl as mec;
            use crate::schema::movie_events::dsl as me;

            // Insert the event row for this movie
            diesel::insert_into(me::movie_events)
                .values((
                    me::movie_id.eq(id),
                    me::submitter_id.eq(admin.user_id),
                    me::time_minutes.eq(event.time_minutes),
                    me::duration_minutes.eq(event.duration_minutes),
                    me::comment.eq(event.comment.as_str()),
                    me::verified.eq(event.verified),
                ))
                .execute(c)?;

            // Fetch the most recent event id for this movie
            let event_id: i32 = me::movie_events
                .filter(me::movie_id.eq(id))
                .order(me::id.desc())
                .select(me::id)
                .first::<i32>(c)?;

            // Link categories to this event (must already exist)
            for cat_name in &event.categories {
                let cid: i32 = ec::event_categories
                    .filter(ec::name.eq(cat_name))
                    .select(ec::id)
                    .first::<i32>(c)?;
                diesel::insert_into(mec::movie_event_categories)
                    .values((
                        mec::movie_event_id.eq(event_id),
                        mec::event_category_id.eq(cid),
                    ))
                    .on_conflict((mec::movie_event_id, mec::event_category_id))
                    .do_nothing()
                    .execute(c)?;
            }

            Ok(event_id)
        })
        .await;

    match result {
        Ok(event_id) => Json(serde_json::json!({ "ok": true, "event_id": event_id })),
        Err(e) => Json(serde_json::json!({ "ok": false, "error": format!("{}", e) })),
    }
}
