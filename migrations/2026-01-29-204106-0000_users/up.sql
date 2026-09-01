-- Your SQL goes here

CREATE TABLE role (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL COLLATE NOCASE UNIQUE
);

INSERT INTO role (name) VALUES
    ("admin"),
    ("curator"),
    ("member");


CREATE TABLE users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 255),
    email TEXT NOT NULL CHECK (email = lower(email)),
    password TEXT NOT NULL CHECK (password LIKE '$argon2id$%'),
    admin_access BOOLEAN NOT NULL DEFAULT 0 CHECK (admin_access IN (0, 1)),
    role_id INTEGER NOT NULL REFERENCES role(id),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);

-- Should be commented out or removed for production!!!
-- Yes, it's the hastings direct Telephone number... the jingle is catchy ok.
--
-- Generate the HASH with argon2 CLI:
--   SALT="$(openssl rand -base64 16)"
--   echo -n 'dev_password' | argon2 "$SALT" -id -e -t 3 -m 15 -p 1
INSERT INTO users (name, email, password, admin_access, role_id) VALUES (
    'Mr Test3', 'test3@test3.com', '$argon2id$v=19$m=32768,t=3,p=1$eHZIMVBhZFl4ZnRQRHR5UG5Fd3dnQT09$41SaoSwWOlMtYCgd74lGedYzA/330VKVU8j+k5C00O4', 1, 1
);
