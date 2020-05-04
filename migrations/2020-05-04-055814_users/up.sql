CREATE TABLE users (
    id         INTEGER NOT NULL PRIMARY KEY,
    username   TEXT    NOT NULL UNIQUE,
    password   TEXT    NOT NULL,
    name       TEXT    NOT NULL,
    role       TEXT    NOT NULL,
    CHECK (role IN ('admin', 'author', 'tutor', 'student'))
);
