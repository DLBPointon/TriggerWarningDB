///
/// !!! PLEASE NOTE THAT THIS WHOLE ENDPOINTS DIRECTORY IS AI GENERATED
/// !!! HENCE WHY IT IS IN ANOTHER FOLDER
/// !!! I needed a way to bulk import a json, the way i made relied on
/// !!! populating each table independently... not great
/// !!! I have read it through and understand what is happening however!
///
use diesel::prelude::*;
use rocket::serde::json::{Json, serde_json};

use crate::DbConn;
use crate::auth::auth::AdminGuard;
use crate::models::movie_models::SeedRequest;

#[post("/api/seed_json", format = "json", data = "<payload>")]
pub async fn seed_json(
    _admin: AdminGuard,
    conn: DbConn,
    payload: Json<SeedRequest>,
) -> Json<serde_json::Value> {
    let req = payload.into_inner();
    let result: Result<(), diesel::result::Error> = conn
        .run(move |c| {
            // Upsert helpers using Diesel's query builder
            fn upsert_studio(
                c: &mut diesel::sqlite::SqliteConnection,
                name_val: &str,
            ) -> diesel::QueryResult<i32> {
                use crate::schema::studios::dsl as s;
                diesel::insert_into(s::studios)
                    .values((s::name.eq(name_val),))
                    .on_conflict(s::name)
                    .do_nothing()
                    .execute(c)?;
                s::studios
                    .filter(s::name.eq(name_val))
                    .select(s::id)
                    .first::<i32>(c)
            }

            fn upsert_director(
                c: &mut diesel::sqlite::SqliteConnection,
                name_val: &str,
            ) -> diesel::QueryResult<i32> {
                use crate::schema::directors::dsl as d;
                diesel::insert_into(d::directors)
                    .values((d::name.eq(name_val),))
                    .on_conflict(d::name)
                    .do_nothing()
                    .execute(c)?;
                d::directors
                    .filter(d::name.eq(name_val))
                    .select(d::id)
                    .first::<i32>(c)
            }

            fn upsert_writer(
                c: &mut diesel::sqlite::SqliteConnection,
                name_val: &str,
            ) -> diesel::QueryResult<i32> {
                use crate::schema::writers::dsl as w;
                diesel::insert_into(w::writers)
                    .values((w::name.eq(name_val),))
                    .on_conflict(w::name)
                    .do_nothing()
                    .execute(c)?;
                w::writers
                    .filter(w::name.eq(name_val))
                    .select(w::id)
                    .first::<i32>(c)
            }

            // Resolve genre (must exist, no creation)
            fn resolve_genre(
                c: &mut diesel::sqlite::SqliteConnection,
                name_val: &str,
            ) -> diesel::QueryResult<i32> {
                use crate::schema::genres::dsl as g;
                g::genres
                    .filter(g::name.eq(name_val))
                    .select(g::id)
                    .first::<i32>(c)
            }

            // Resolve event category (must exist, no creation)
            fn resolve_event_category(
                c: &mut diesel::sqlite::SqliteConnection,
                name_val: &str,
            ) -> diesel::QueryResult<i32> {
                use crate::schema::event_categories::dsl as ec;
                ec::event_categories
                    .filter(ec::name.eq(name_val))
                    .select(ec::id)
                    .first::<i32>(c)
            }

            for m in req.movies {
                // Resolve certification ID from string
                let cert_id = {
                    use crate::schema::certification::dsl as cert;
                    cert::certification
                        .filter(cert::name.eq(&m.certification))
                        .select(cert::id)
                        .first::<i32>(c)?
                };

                // Insert movie
                {
                    use crate::schema::movies::dsl as mv;
                    diesel::insert_into(mv::movies)
                        .values((
                            mv::title.eq(&m.title),
                            mv::runtime_minutes.eq(m.runtime_minutes),
                            mv::synopsis.eq(m.synopsis.as_deref()),
                            mv::release_year.eq(m.release_year),
                            mv::poster_url.eq(m.poster_url.as_deref()),
                            mv::imdb_code.eq(m.imdb_code),
                            mv::certification.eq(cert_id),
                        ))
                        .execute(c)?;
                    ()
                }

                // Get movie id (latest by title, avoids last_insert_rowid issues)
                use crate::schema::movies::dsl as mv;
                let movie_id: i32 = mv::movies
                    .filter(mv::title.eq(&m.title))
                    .order(mv::id.desc())
                    .select(mv::id)
                    .first::<i32>(c)?;

                // Studios
                for sname in &m.studios {
                    let sid = upsert_studio(c, sname)?;
                    use crate::schema::movie_studios::dsl as ms;
                    diesel::insert_into(ms::movie_studios)
                        .values((ms::movie_id.eq(movie_id), ms::studio_id.eq(sid)))
                        .on_conflict((ms::movie_id, ms::studio_id))
                        .do_nothing()
                        .execute(c)?;
                }

                // Directors
                for dname in &m.directors {
                    let did = upsert_director(c, dname)?;
                    use crate::schema::movie_directors::dsl as md;
                    diesel::insert_into(md::movie_directors)
                        .values((md::movie_id.eq(movie_id), md::director_id.eq(did)))
                        .on_conflict((md::movie_id, md::director_id))
                        .do_nothing()
                        .execute(c)?;
                }

                // Writers
                for wname in &m.writers {
                    let wid = upsert_writer(c, wname)?;
                    use crate::schema::movie_writers::dsl as mw;
                    diesel::insert_into(mw::movie_writers)
                        .values((mw::movie_id.eq(movie_id), mw::writer_id.eq(wid)))
                        .on_conflict((mw::movie_id, mw::writer_id))
                        .do_nothing()
                        .execute(c)?;
                }

                // Genres (must exist)
                for gname in &m.genres {
                    let gid = resolve_genre(c, gname)?;
                    use crate::schema::movie_genres::dsl as mg;
                    diesel::insert_into(mg::movie_genres)
                        .values((mg::movie_id.eq(movie_id), mg::genre_id.eq(gid)))
                        .on_conflict((mg::movie_id, mg::genre_id))
                        .do_nothing()
                        .execute(c)?;
                }

                // Events
                for ev in &m.events {
                    {
                        use crate::schema::movie_events::dsl as me;
                        diesel::insert_into(me::movie_events)
                            .values((
                                me::movie_id.eq(movie_id),
                                me::submitter_id.eq(req.submitter_id),
                                me::time_minutes.eq(ev.time_minutes),
                                me::duration_minutes.eq(ev.duration_minutes),
                                me::comment.eq(ev.comment.as_str()),
                                me::verified.eq(ev.verified),
                            ))
                            .execute(c)?;
                    }

                    // Get the latest event id for this movie
                    use crate::schema::movie_events::dsl as me;
                    let event_id: i32 = me::movie_events
                        .filter(me::movie_id.eq(movie_id))
                        .order(me::id.desc())
                        .select(me::id)
                        .first::<i32>(c)?;

                    // Event categories (must exist)
                    for cat_name in &ev.categories {
                        let cid = resolve_event_category(c, cat_name)?;
                        use crate::schema::movie_event_categories::dsl as mec;
                        diesel::insert_into(mec::movie_event_categories)
                            .values((
                                mec::movie_event_id.eq(event_id),
                                mec::event_category_id.eq(cid),
                            ))
                            .on_conflict((mec::movie_event_id, mec::event_category_id))
                            .do_nothing()
                            .execute(c)?;
                    }
                }
            }

            Ok(())
        })
        .await;

    match result {
        Ok(_) => Json(serde_json::json!({ "ok": true })),
        Err(e) => Json(serde_json::json!({ "ok": false, "error": format!("{}", e) })),
    }
}
