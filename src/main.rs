use std::env;
mod auth;
mod crud;
mod endpoints;
mod models;
mod schema;

#[macro_use]
extern crate rocket;
use crate::auth::auth::{AdminGuard, AuthConfig};
use crate::crud::{
    app_crud::{app_map, get_app_data},
    movie_crud::create_movie,
    user_crud::find_user_by_email,
};
use crate::schema::event_categories::dsl as event_cats;

use crate::endpoints::add_event::api_add_event;
use crate::endpoints::event_categories::{list_event_categories, list_genre_categories};
use crate::endpoints::movie_details::detail;
use crate::endpoints::seed::seed_json;
use crate::models::{
    movie_models::{Movies, NewMovie},
    user_models::{LoginRequest, LoginResponse, User},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rocket::State;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::serde::json::{Json, serde_json};
use rocket_dyn_templates::{Template, context};
use rocket_sync_db_pools::{database, diesel};

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

///
/// LOGIN
///
#[post("/api/login", format = "json", data = "<creds>")]
async fn login(
    creds: Json<LoginRequest>,
    conn: DbConn,
    cfg: &State<AuthConfig>,
) -> Result<Json<LoginResponse>, Status> {
    let email = creds.email.clone();
    let user_res = conn.run(move |c| find_user_by_email(c, &email)).await;

    let user: User = match user_res {
        Ok(u) => u,
        Err(_) => return Err(Status::Unauthorized),
    };

    // Password field in DB must be an argon2 hash (PHC string).
    let parsed_hash = PasswordHash::new(&user.password).map_err(|_| Status::InternalServerError)?;
    let verified = Argon2::default()
        .verify_password(creds.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !verified {
        return Err(Status::Unauthorized);
    }

    let token = cfg
        .encode_token(user.user_id, user.admin_access)
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(LoginResponse { token }))
}

///
/// MOVIES
///
#[post("/movies", format = "json", data = "<movie>")]
async fn api_create_movie(_admin: AdminGuard, movie: Json<NewMovie>, conn: DbConn) -> Json<Movies> {
    conn.run(move |c| create_movie(c, &movie))
        .await
        .map(Json)
        .expect("Failed to create movie")
}

#[get(
    "/movies?<page>&<per_page>&<title>&<year>&<studio>&<director>&<writer>&<genre>&<event_category>"
)]
async fn list(
    conn: DbConn,
    page: Option<i64>,
    per_page: Option<i64>,
    title: Option<String>,
    year: Option<i32>,
    studio: Option<String>,
    director: Option<String>,
    writer: Option<String>,
    genre: Option<String>,
    event_category: Option<String>,
) -> Template {
    let pagination_ctx = conn
        .run(move |c| {
            let page = page.unwrap_or(1).max(1);
            let per_page = per_page.unwrap_or(12).clamp(1, 100);
            let offset = (page - 1) * per_page;
            use crate::schema::movie_events::dsl as me;
            use crate::schema::movies::dsl as mv;
            use diesel::prelude::*;

            // Build base movies query with optional multi-field search
            use crate::schema as s;
            use diesel::dsl::exists;

            let mut count_query = mv::movies.into_boxed();
            let mut page_query = mv::movies.into_boxed();

            // Title filter
            if let Some(t) = &title {
                let like = format!("%{}%", t);
                count_query = count_query.filter(mv::title.like(like.clone()));
                page_query = page_query.filter(mv::title.like(like));
            }

            // Year filter
            if let Some(y) = year {
                count_query = count_query.filter(mv::release_year.eq(y));
                page_query = page_query.filter(mv::release_year.eq(y));
            }

            // Studio filter
            if let Some(st) = &studio {
                let like = format!("%{}%", st);
                let cond = exists(
                    s::movie_studios::table
                        .inner_join(
                            s::studios::table.on(s::movie_studios::studio_id.eq(s::studios::id)),
                        )
                        .filter(s::movie_studios::movie_id.eq(mv::id))
                        .filter(s::studios::name.like(like))
                        .select(s::movie_studios::movie_id),
                );
                count_query = count_query.filter(cond.clone());
                page_query = page_query.filter(cond);
            }

            // Director filter
            if let Some(dir) = &director {
                let like = format!("%{}%", dir);
                let cond = exists(
                    s::movie_directors::table
                        .inner_join(
                            s::directors::table
                                .on(s::movie_directors::director_id.eq(s::directors::id)),
                        )
                        .filter(s::movie_directors::movie_id.eq(mv::id))
                        .filter(s::directors::name.like(like))
                        .select(s::movie_directors::movie_id),
                );
                count_query = count_query.filter(cond.clone());
                page_query = page_query.filter(cond);
            }

            // Writer filter
            if let Some(wr) = &writer {
                let like = format!("%{}%", wr);
                let cond = exists(
                    s::movie_writers::table
                        .inner_join(
                            s::writers::table.on(s::movie_writers::writer_id.eq(s::writers::id)),
                        )
                        .filter(s::movie_writers::movie_id.eq(mv::id))
                        .filter(s::writers::name.like(like))
                        .select(s::movie_writers::movie_id),
                );
                count_query = count_query.filter(cond.clone());
                page_query = page_query.filter(cond);
            }

            // Genre filter
            if let Some(gn) = &genre {
                let like = format!("%{}%", gn);
                let cond = exists(
                    s::movie_genres::table
                        .inner_join(
                            s::genres::table.on(s::movie_genres::genre_id.eq(s::genres::id)),
                        )
                        .filter(s::movie_genres::movie_id.eq(mv::id))
                        .filter(s::genres::name.like(like))
                        .select(s::movie_genres::movie_id),
                );
                count_query = count_query.filter(cond.clone());
                page_query = page_query.filter(cond);
            }

            // Event category (trauma type) filter
            if let Some(ec) = &event_category {
                let name = ec.clone();
                let event_ids = s::movie_event_categories::table
                    .inner_join(s::event_categories::table)
                    .filter(s::event_categories::name.eq(name))
                    .select(s::movie_event_categories::movie_event_id);
                let cond = exists(
                    s::movie_events::table
                        .filter(s::movie_events::movie_id.eq(mv::id))
                        .filter(s::movie_events::id.eq_any(event_ids))
                        .select(s::movie_events::id),
                );
                count_query = count_query.filter(cond.clone());
                page_query = page_query.filter(cond);
            }

            // Total count for filtered query (use independent boxed query to avoid move issues)
            let total_count: i64 = count_query.count().get_result::<i64>(c).unwrap_or(0);

            // Page results (use independent boxed query to avoid move issues)
            let ms = page_query
                .order(mv::id.desc())
                .limit(per_page)
                .offset(offset)
                .load::<crate::models::movie_models::Movies>(c)?;

            // For each movie, compute events_count and events_per_hour
            let items = ms
                .into_iter()
                .map(|m| {
                    let count: i64 = me::movie_events
                        .filter(me::movie_id.eq(m.id))
                        .count()
                        .get_result::<i64>(c)
                        .unwrap_or(0);

                    let eph = match m.runtime_minutes {
                        rs if rs > 0 => Some((count as f64 * 60.0) / (rs as f64)),
                        _ => None,
                    };

                    serde_json::json!({
                        "id": m.id,
                        "general_notes": m.general_notes,
                        "title": m.title,
                        "certification": m.certification,
                        "runtime_minutes": m.runtime_minutes,
                        "release_year": m.release_year,
                        "poster_url": m.poster_url,
                        "events_count": count,
                        "events_per_hour": eph,
                    })
                })
                .collect::<Vec<_>>();

            // total_count computed above from filtered query

            // Load genres list for advanced search dropdown
            let genres_list = {
                use crate::schema::genres::dsl as g;
                g::genres
                    .order(g::name.asc())
                    .select(g::name)
                    .load::<String>(c)
                    .unwrap_or_default()
            };

            let event_categories_list = {
                event_cats::event_categories
                    .order(event_cats::name.asc())
                    .select(event_cats::name)
                    .load::<String>(c)
                    .unwrap_or_default()
            };

            Ok::<serde_json::Value, diesel::result::Error>(serde_json::json!({
                "items": items,
                "page": page,
                "per_page": per_page,
                "total_count": total_count,
                "has_prev": page > 1,
                "has_next": (offset + per_page) < total_count,
                "title": title,
                "year": year,
                "studio": studio,
                "director": director,
                "writer": writer,
                "genre": genre,
                "genres": genres_list,
                "event_category": event_category,
                "event_categories": event_categories_list
            }))
        })
        .await
        .ok();

    Template::render(
        "movie_list",
        context! { page: pagination_ctx, current_page: "movie_list" },
    )
}

///
/// ABOUT
///
#[get("/about")]
async fn about(conn: DbConn) -> Template {
    let app = conn.run(|c| get_app_data(c)).await.ok();
    let app_ctx = app.map(|a| app_map(&a));
    Template::render(
        "about",
        context! { app_info: app_ctx, current_page: "about"},
    )
}

///
/// INDEX
///
#[get("/")]
async fn hello(conn: DbConn) -> Template {
    let app = conn.run(|c| get_app_data(c)).await.ok();
    let app_ctx = app.map(|a| app_map(&a));
    Template::render(
        "index",
        context! { app_info: app_ctx, current_page: "index" },
    )
}

///
/// LOGIN PAGE
///
#[get("/login")]
async fn login_page(conn: DbConn) -> Template {
    let app = conn.run(|c| get_app_data(c)).await.ok();
    let app_ctx = app.map(|a| app_map(&a));
    Template::render(
        "login",
        context! { app_info: app_ctx, current_page: "login" },
    )
}

///
/// PROFILE PAGE
///
#[get("/profile")]
async fn profile_page(conn: DbConn) -> Template {
    let app = conn.run(|c| get_app_data(c)).await.ok();
    let app_ctx = app.map(|a| app_map(&a));
    Template::render(
        "profile",
        context! { app_info: app_ctx, current_page: "profile" },
    )
}

///
/// CATEGORY EXPLAINER
///
#[get("/category_explainer")]
async fn category_explainer(conn: DbConn) -> Template {
    let app = conn.run(|c| get_app_data(c)).await.ok();
    let app_ctx = app.map(|a| app_map(&a));
    Template::render(
        "category_explainer",
        context! { app_info: app_ctx, current_page: "categories" },
    )
}

///
/// Error Catching
///
#[catch(404)]
async fn not_found() -> Template {
    Template::render(
        "error",
        context! {page_info: "404", error_message: "This page does not exist! It's likely that it will in the future though!"},
    )
}

///
/// 404 Catch
///
#[catch(500)]
async fn err_found() -> Template {
    Template::render(
        "error",
        context! {page_info: "500", error_message: "Internal Error, the developer probably forgot a variable somewhere!"},
    )
}

///
/// MAIN LAUNCHER
///
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let auth_cfg = AuthConfig {
        jwt_secret,
        token_ttl_hours: 24, // for example
    };

    let figment = rocket::Config::figment().merge((
        "databases.sqlite_database",
        rocket::Config::from(rocket::build().figment()),
    ));

    rocket::custom(figment)
        .manage(auth_cfg)
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                hello,
                about,
                login_page,
                profile_page,
                login,
                api_create_movie,
                api_add_event,
                list,
                detail,
                seed_json,
                category_explainer,
                list_event_categories,
                list_genre_categories
            ],
        )
        .mount("/static", FileServer::from("static"))
        .mount("/images", FileServer::from("images"))
        .register("/", catchers![not_found, err_found])
        .launch()
        .await?;

    Ok(())
}
