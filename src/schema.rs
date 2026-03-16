// @generated automatically by Diesel CLI.

diesel::table! {
    app_info (app_id) {
        app_id -> Nullable<Integer>,
        app_name -> Text,
        app_semantic_version -> Text,
        app_named_version -> Text,
    }
}

diesel::table! {
    directors (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    event_categories (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    genres (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    movie_directors (movie_id, director_id) {
        movie_id -> Integer,
        director_id -> Integer,
    }
}

diesel::table! {
    movie_event_categories (movie_event_id, event_category_id) {
        movie_event_id -> Integer,
        event_category_id -> Integer,
    }
}

diesel::table! {
    movie_events (id) {
        id -> Integer,
        movie_id -> Integer,
        submitter_id -> Integer,
        time_minutes -> Integer,
        duration_minutes -> Integer,
        comment -> Nullable<Text>,
        verified -> Bool,
    }
}

diesel::table! {
    movie_genres (movie_id, genre_id) {
        movie_id -> Integer,
        genre_id -> Integer,
    }
}

diesel::table! {
    movie_studios (movie_id, studio_id) {
        movie_id -> Integer,
        studio_id -> Integer,
    }
}

diesel::table! {
    movie_writers (movie_id, writer_id) {
        movie_id -> Integer,
        writer_id -> Integer,
    }
}

diesel::table! {
    movies (id) {
        id -> Integer,
        general_notes -> Nullable<Text>,
        title -> Text,
        runtime_minutes -> Integer,
        synopsis -> Nullable<Text>,
        release_year -> Integer,
        poster_url -> Nullable<Text>,
        imdb_code -> Text,
        certification -> Text,
    }
}

diesel::table! {
    studios (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    tv_shows (id) {
        id -> Integer,
        show_title -> Text,
        season_count -> Integer,
        episode_count -> Integer,
        synopsis -> Nullable<Text>,
        release_year -> Integer,
        imdb_code -> Text,
        certification -> Text,
        poster_url -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
        admin_access -> Bool,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    writers (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(movie_directors -> directors (director_id));
diesel::joinable!(movie_directors -> movies (movie_id));
diesel::joinable!(movie_event_categories -> event_categories (event_category_id));
diesel::joinable!(movie_event_categories -> movie_events (movie_event_id));
diesel::joinable!(movie_events -> movies (movie_id));
diesel::joinable!(movie_events -> users (submitter_id));
diesel::joinable!(movie_genres -> genres (genre_id));
diesel::joinable!(movie_genres -> movies (movie_id));
diesel::joinable!(movie_studios -> movies (movie_id));
diesel::joinable!(movie_studios -> studios (studio_id));
diesel::joinable!(movie_writers -> movies (movie_id));
diesel::joinable!(movie_writers -> writers (writer_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_info,
    directors,
    event_categories,
    genres,
    movie_directors,
    movie_event_categories,
    movie_events,
    movie_genres,
    movie_studios,
    movie_writers,
    movies,
    studios,
    tv_shows,
    users,
    writers,
);
