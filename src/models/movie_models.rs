use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::movies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Movies {
    pub id: i32,
    pub general_notes: Option<String>,
    pub title: String,
    pub runtime_minutes: i32,
    pub synopsis: Option<String>,
    pub release_year: i32,
    pub poster_url: Option<String>,
    pub imdb_code: String,
    pub certification: i32,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::movies)]
pub struct NewMovie {
    pub title: String,
    pub general_notes: Option<String>,
    pub runtime_minutes: i32,
    pub synopsis: Option<String>,
    pub release_year: i32,
    pub poster_url: Option<String>,
    pub imdb_code: String,
    pub certification: i32,
}

#[derive(Deserialize)]
pub struct SeedEvent {
    pub time_minutes: i32,
    pub duration_minutes: i32,
    pub comment: String,
    pub categories: Vec<String>,
    pub verified: bool,
}

#[derive(Deserialize)]
pub struct SeedMovie {
    pub title: String,
    pub certification: String,
    pub runtime_minutes: i32,
    pub synopsis: Option<String>,
    pub release_year: i32,
    pub poster_url: Option<String>,
    pub imdb_code: String,
    pub studios: Vec<String>,
    pub directors: Vec<String>,
    pub writers: Vec<String>,
    pub genres: Vec<String>,
    pub events: Vec<SeedEvent>,
}

#[derive(Deserialize)]
pub struct SeedRequest {
    pub movies: Vec<SeedMovie>,
    pub submitter_id: i32,
}
