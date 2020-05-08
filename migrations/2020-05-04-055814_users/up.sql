CREATE TABLE users (
    id         INTEGER NOT NULL PRIMARY KEY,
    username   TEXT    NOT NULL UNIQUE,
    password   TEXT    NOT NULL,
    name       TEXT    NOT NULL,
    role       TEXT    NOT NULL,
    active     BOOLEAN NOT NULL DEFAULT FALSE,
    code       TEXT    NOT NULL DEFAULT '',
    CHECK (role IN ('admin', 'author', 'tutor', 'student'))
);
