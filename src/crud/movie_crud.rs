use crate::models::movie_models::{Movies, NewMovie};
use crate::schema::movies;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

///
/// API: Create movie via api request
///
pub fn create_movie(conn: &mut SqliteConnection, new_movie: &NewMovie) -> QueryResult<Movies> {
    conn.transaction(|c| {
        diesel::insert_into(movies::table)
            .values(new_movie)
            .execute(c)?;
        movies::table.order(movies::id.desc()).first::<Movies>(c)
    })
}
