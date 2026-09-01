CREATE TABLE app_info (
    app_id INTEGER PRIMARY KEY CHECK (app_id = 1),
    app_name TEXT NOT NULL,
    app_semantic_version TEXT NOT NULL,
    app_named_version TEXT NOT NULL,
    homepage_welcome_banner TEXT NOT NULL,
    homepage_welcome_text TEXT NOT NULL,
    about_us_text TEXT NOT NULL,
    goals_text TEXT NOT NULL
);

INSERT INTO app_info (
    app_id, app_name, app_semantic_version,
    app_named_version, homepage_welcome_banner,
    homepage_welcome_text, about_us_text, goals_text
) VALUES (
    '1',
    'Trigger Warning',
    '0.1.0',
    'Battlestar Galactica',
    'Welcome to Trigger Warning!',
    'UNINITIALISED',
    'UNINITIALISED',
    'UNINITIALISED'
);
