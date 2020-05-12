CREATE TABLE courses (
    id         INTEGER NOT NULL PRIMARY KEY,
    code       TEXT    NOT NULL UNIQUE,
    title      TEXT    NOT NULL,
    author_id  INTEGER NOT NULL REFERENCES users(id),
    tutor_id   INTEGER NOT NULL REFERENCES users(id),
    active     BOOLEAN NOT NULL DEFAULT TRUE
);
