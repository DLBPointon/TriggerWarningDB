/*
Core Movies DB, admitedly I used a fair amount of AI to write this
Not from scratch i'd like to point out, more that it converted an old postgresql schema I wrote in (2020-ish time)
into sqlite3. A perfect job for AI i think.
*/

/* Redesigning this DB again i think i'd go with a compound key of Movie title + release year + imdb_code
like i've done with the uniqueness check.
It would just make it easier to search */

PRAGMA foreign_keys = ON; /* literally turns on foreign keys */

/***************
  Core table
***************/
CREATE TABLE IF NOT EXISTS movies (
  id              INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  general_notes   TEXT,
  title           TEXT NOT NULL,
  runtime_minutes INTEGER NOT NULL,
  synopsis        TEXT,
  release_year    INTEGER NOT NULL,
  poster_url      TEXT,
  imdb_code       TEXT NOT NULL,
  certification   TEXT NOT NULL,
  CHECK (certification in ('U', 'PG', '12A', '12', '15', '18', 'R18'))
);

-- Ensure natural key uniqueness across title, release_year, and imdb_code
CREATE UNIQUE INDEX IF NOT EXISTS idx_movies_unique_title_year_imdb
  ON movies(title, release_year, imdb_code);

/***************
  Genres (case-insensitive unique)
***************/
CREATE TABLE IF NOT EXISTS genres (
  id   INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL COLLATE NOCASE UNIQUE
);

/* link multiple genres to movie based on primary key */
CREATE TABLE IF NOT EXISTS movie_genres (
  movie_id INTEGER NOT NULL REFERENCES movies(id) ON DELETE CASCADE,
  genre_id INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
  PRIMARY KEY (movie_id, genre_id)
);
CREATE INDEX IF NOT EXISTS idx_movie_genres_movie ON movie_genres(movie_id);
CREATE INDEX IF NOT EXISTS idx_movie_genres_genre ON movie_genres(genre_id);

/* Seed genres */
INSERT INTO genres (name) VALUES
  ('Action'),
  ('Adventure'),
  ('Animation'),
  ('Comedy'),
  ('Crime'),
  ('Documentary'),
  ('Drama'),
  ('Fantasy'),
  ('Historical'),
  ('Horror'),
  ('Mystery'),
  ('Musical'),
  ('Romance'),
  ('Sci-Fi'),
  ('Thriller'),
  ('War'),
  ('Western')
ON CONFLICT(name) DO NOTHING;

/* Triggers to stop any editing of the genre values */

CREATE TRIGGER IF NOT EXISTS trg_genres_block_insert
BEFORE INSERT ON genres
BEGIN
  SELECT RAISE(FAIL, 'genres are fixed; insertion is not allowed');
END;

CREATE TRIGGER IF NOT EXISTS trg_genres_block_update
BEFORE UPDATE ON genres
BEGIN
  SELECT RAISE(FAIL, 'genres are fixed; updates are not allowed');
END;

CREATE TRIGGER IF NOT EXISTS trg_genres_block_delete
BEFORE DELETE ON genres
BEGIN
  SELECT RAISE(FAIL, 'genres are fixed; deletion is not allowed');
END;

/***************
  Event_categories (case-insensitive unique)
***************/

CREATE TABLE IF NOT EXISTS event_categories (
  id   INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL COLLATE NOCASE UNIQUE
);

/* Actual events table */
CREATE TABLE IF NOT EXISTS movie_events (
  id               INTEGER PRIMARY KEY NOT NULL,
  movie_id         INTEGER NOT NULL REFERENCES movies(id)      ON DELETE CASCADE,
  submitter_id     INTEGER NOT NULL REFERENCES users(user_id)  ON DELETE RESTRICT,
  time_minutes     INTEGER NOT NULL DEFAULT 0,
  duration_minutes INTEGER NOT NULL DEFAULT 5,
  comment          TEXT,
  verified         BOOL NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_movie_events_movie     ON movie_events(movie_id);
CREATE INDEX IF NOT EXISTS idx_movie_events_submitter ON movie_events(submitter_id);

/* linker table for movie_event and event_category */
CREATE TABLE IF NOT EXISTS movie_event_categories (
  movie_event_id    INTEGER NOT NULL REFERENCES movie_events(id)     ON DELETE CASCADE,
  event_category_id INTEGER NOT NULL REFERENCES event_categories(id) ON DELETE RESTRICT,
  PRIMARY KEY (movie_event_id, event_category_id)
);
CREATE INDEX IF NOT EXISTS idx_movie_event_categories_event ON movie_event_categories(movie_event_id);
CREATE INDEX IF NOT EXISTS idx_movie_event_categories_cat   ON movie_event_categories(event_category_id);


/* Seed values for the event_categories */
INSERT INTO event_categories (name) VALUES
    ("Birth Event"),
    ("Baby Loss"),
    ("Parent Loss"),
    ("Sexual Violence"),
    ("Gender-based Violence"),
    ("Car Crash"),
    ("Psychological Abuse"),
    ("Physical Abuse"),
    ("Other")
ON CONFLICT(name) DO NOTHING;

/* Lock the aboves from editing */
CREATE TRIGGER IF NOT EXISTS trg_event_categories_block_insert
BEFORE INSERT ON event_categories
BEGIN
  SELECT RAISE(FAIL, 'event_categories are fixed; insertion is not allowed');
END;

CREATE TRIGGER IF NOT EXISTS trg_event_categories_block_update
BEFORE UPDATE ON event_categories
BEGIN
  SELECT RAISE(FAIL, 'event_categories are fixed; updates are not allowed');
END;

CREATE TRIGGER IF NOT EXISTS trg_event_categories_block_delete
BEFORE DELETE ON event_categories
BEGIN
  SELECT RAISE(FAIL, 'event_categories are fixed; deletion is not allowed');
END;

/* Studio, Director and Writer tables */
CREATE TABLE IF NOT EXISTS studios (
  id   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL COLLATE NOCASE UNIQUE
);

CREATE TABLE IF NOT EXISTS directors (
  id   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL COLLATE NOCASE UNIQUE
);

CREATE TABLE IF NOT EXISTS writers (
  id   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL COLLATE NOCASE UNIQUE
);

/* Linker tables */

CREATE TABLE IF NOT EXISTS movie_studios (
  movie_id  INTEGER NOT NULL REFERENCES movies(id)  ON DELETE CASCADE,
  studio_id INTEGER NOT NULL REFERENCES studios(id) ON DELETE RESTRICT,
  PRIMARY KEY (movie_id, studio_id)
);

CREATE INDEX IF NOT EXISTS idx_movie_studios_movie  ON movie_studios(movie_id);
CREATE INDEX IF NOT EXISTS idx_movie_studios_studio ON movie_studios(studio_id);

CREATE TABLE IF NOT EXISTS movie_directors (
  movie_id    INTEGER NOT NULL REFERENCES movies(id)    ON DELETE CASCADE,
  director_id INTEGER NOT NULL REFERENCES directors(id) ON DELETE RESTRICT,
  PRIMARY KEY (movie_id, director_id)
);

CREATE INDEX IF NOT EXISTS idx_movie_directors_movie    ON movie_directors(movie_id);
CREATE INDEX IF NOT EXISTS idx_movie_directors_director ON movie_directors(director_id);

CREATE TABLE IF NOT EXISTS movie_writers (
  movie_id  INTEGER NOT NULL REFERENCES movies(id)  ON DELETE CASCADE,
  writer_id INTEGER NOT NULL REFERENCES writers(id) ON DELETE RESTRICT,
  PRIMARY KEY (movie_id, writer_id)
);

CREATE INDEX IF NOT EXISTS idx_movie_writers_movie  ON movie_writers(movie_id);
CREATE INDEX IF NOT EXISTS idx_movie_writers_writer ON movie_writers(writer_id);

/* Enforce basic shape: starts with 'tt' */
/* I need to read more about the code schema */
-- CREATE TRIGGER IF NOT EXISTS trg_movies_check_imdb
-- BEFORE INSERT ON movies
-- BEGIN
--   SELECT CASE
--     WHEN NEW.imdb_code IS NOT NULL
--      AND (substr(NEW.imdb_code,1,2) != 'tt' /* must start with `tt` */
--        OR length(NEW.imdb_code) < 8         /* must have at least 2 letters and 7 digits */
--     THEN RAISE(FAIL, 'Invalid imdb_code format, expected tt followed by at least 7 digits')
--   END;
-- END;
