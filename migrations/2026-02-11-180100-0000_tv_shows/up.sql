
PRAGMA foreign_keys = ON; /* literally turns on foreign keys */

/***************
  Core table
***************/
CREATE TABLE IF NOT EXISTS tv_shows (
    id              INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    show_title      TEXT NOT NULL,
    season_count    INTEGER NOT NULL,
    episode_count   INTEGER NOT NULL,
    synopsis        TEXT,
    release_year    INTEGER NOT NULL,
    imdb_code       TEXT NOT NULL,
    certification   TEXT NOT NULL,
    poster_url      TEXT NOT NULL,
    CHECK (certification in ('U', 'PG', '12A', '12', '15', '18', 'R18'))
    );

-- Ensure natural key uniqueness across title, release_year, and imdb_code
CREATE UNIQUE INDEX IF NOT EXISTS idx_tv_shows_unique_title_year_imdb
  ON tv_shows(show_title, release_year, imdb_code);
