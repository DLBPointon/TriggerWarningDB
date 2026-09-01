///
/// !!! PLEASE NOTE THAT THIS WHOLE ENDPOINTS DIRECTORY IS AI GENERATED
/// !!! HENCE WHY IT IS IN ANOTHER FOLDER
///
use crate::DbConn;
use diesel::prelude::*;
use rocket::serde::json::serde_json;
use rocket_dyn_templates::{Template, context};

/// Full movie details endpoint handler, moved from `main.rs`.
/// Returns the base movie plus genres, studios, directors, writers,
/// and events with their category names.
#[get("/movies/<id>")]
pub async fn detail(id: i32, conn: DbConn) -> Template {
    // Fetch and aggregate all related data in a single DB run.
    let data = conn
        .run(move |c| {
            use crate::schema::directors::dsl as d;
            use crate::schema::event_categories::dsl as ec;
            use crate::schema::genres::dsl as g;
            use crate::schema::movie_directors::dsl as md;
            use crate::schema::movie_event_categories::dsl as mec;
            use crate::schema::movie_events::dsl as me;
            use crate::schema::movie_genres::dsl as mg;
            use crate::schema::movie_studios::dsl as ms;
            use crate::schema::movie_writers::dsl as mw;
            use crate::schema::movies::dsl as mv;
            use crate::schema::studios::dsl as s;
            use crate::schema::writers::dsl as w;

            // Base movie (optional if not found)
            let m = mv::movies
                .filter(mv::id.eq(id))
                .first::<crate::models::movie_models::Movies>(c)
                .optional()?;

            if let Some(m) = m {
                // Genres (names)
                let genre_names: Vec<String> = mg::movie_genres
                    .inner_join(g::genres.on(mg::genre_id.eq(g::id)))
                    .filter(mg::movie_id.eq(id))
                    .select(g::name)
                    .load::<String>(c)?;

                // Studios (names)
                let studio_names: Vec<String> = ms::movie_studios
                    .inner_join(s::studios.on(ms::studio_id.eq(s::id)))
                    .filter(ms::movie_id.eq(id))
                    .select(s::name)
                    .load::<String>(c)?;

                // Directors (names)
                let director_names: Vec<String> = md::movie_directors
                    .inner_join(d::directors.on(md::director_id.eq(d::id)))
                    .filter(md::movie_id.eq(id))
                    .select(d::name)
                    .load::<String>(c)?;

                // Writers (names)
                let writer_names: Vec<String> = mw::movie_writers
                    .inner_join(w::writers.on(mw::writer_id.eq(w::id)))
                    .filter(mw::movie_id.eq(id))
                    .select(w::name)
                    .load::<String>(c)?;

                // Certification name (from certification table)
                use crate::schema::certification::dsl as cert_dsl;
                let certification_name: String = cert_dsl::certification
                    .filter(cert_dsl::id.eq(m.certification))
                    .select(cert_dsl::name)
                    .first::<String>(c)
                    .unwrap_or_else(|_| "Unknown".to_string());

                // Events rows: (id, movie_id, event_id, time_minutes, duration_minutes, comment)
                // We select specific columns rather than the whole row, since we only need these fields.
                let events_rows = me::movie_events
                    .filter(me::movie_id.eq(id))
                    .order(me::id.asc())
                    .select((
                        me::id,
                        me::time_minutes,
                        me::duration_minutes,
                        me::comment,
                        me::verified,
                    ))
                    .load::<(i32, i32, i32, Option<String>, bool)>(c)
                    .optional()?;

                // Build grouped events by category with counts
                use std::collections::BTreeMap;
                let mut category_map: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();

                if let Some(rows) = events_rows {
                    for (event_id, time_minutes, duration_minutes, comment, verified) in rows {
                        // Resolve category names for this event
                        let category_names: Vec<String> = mec::movie_event_categories
                            .inner_join(ec::event_categories.on(mec::event_category_id.eq(ec::id)))
                            .filter(mec::movie_event_id.eq(event_id))
                            .select(ec::name)
                            .load::<String>(c)
                            .unwrap_or_default();

                        // Event JSON
                        let evt = serde_json::json!({
                            "id": event_id,
                            "time_minutes": time_minutes,
                            "duration_minutes": duration_minutes,
                            "comment": comment,
                            "verified": verified,
                            "categories": category_names
                        });

                        // Place this event into each of its categories (or "Uncategorized")
                        if category_names.is_empty() {
                            category_map
                                .entry("Uncategorized".to_string())
                                .or_default()
                                .push(evt);
                        } else {
                            for cat in category_names {
                                category_map.entry(cat).or_default().push(evt.clone());
                            }
                        }
                    }
                }

                // Convert map into an array for template iteration
                let grouped_events: Vec<serde_json::Value> = category_map
                    .into_iter()
                    .map(|(cat, events)| {
                        serde_json::json!({
                            "category": cat,
                            "count": events.len(),
                            "events": events
                        })
                    })
                    .collect();

                Ok::<
                    Option<(
                        crate::models::movie_models::Movies,
                        Vec<String>,            // genres
                        Vec<String>,            // studios
                        Vec<String>,            // directors
                        Vec<String>,            // writers
                        Vec<serde_json::Value>, // grouped_events
                        String,                 // certification_name
                    )>,
                    diesel::result::Error,
                >(Some((
                    m,
                    genre_names,
                    studio_names,
                    director_names,
                    writer_names,
                    grouped_events,
                    certification_name,
                )))
            } else {
                Ok(None)
            }
        })
        .await
        .ok()
        .flatten();

    // If not found, render with empty context
    let (m, genres, studios, directors, writers, grouped_events, certification_name) = match data {
        Some(tuple) => tuple,
        None => {
            return Template::render(
                "movie_detail",
                context! {
                    movie: None::<serde_json::Value>,
                    grouped_events: None::<Vec<serde_json::Value>>,
                    genres: None::<Vec<String>>,
                    studios: None::<Vec<String>>,
                    directors: None::<Vec<String>>,
                    writers: None::<Vec<String>>
                },
            );
        }
    };

    // Build movie object for template
    let movie_ctx = serde_json::json!({
        "id": m.id,
        "general_notes": m.general_notes,
        "title": m.title,
        "runtime_minutes": m.runtime_minutes,
        "synopsis": m.synopsis,
        "release_year": m.release_year,
        "poster_url": m.poster_url,
        "imdb_code": m.imdb_code,
        "certification": certification_name
    });

    // Compute total events count across all grouped categories
    let events_count: usize = grouped_events
        .iter()
        .map(|grp| grp.get("count").and_then(|c| c.as_u64()).unwrap_or(0) as usize)
        .sum();

    Template::render(
        "movie_detail",
        context! {
            movie: movie_ctx,
            grouped_events: grouped_events,
            events_count: events_count,
            genres: genres,
            studios: studios,
            directors: directors,
            writers: writers,
            current_page: "movie_detail"
        },
    )
}
