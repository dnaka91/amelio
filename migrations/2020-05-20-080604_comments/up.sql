CREATE TABLE comments (
    id         INTEGER NOT NULL PRIMARY KEY,
    ticket_id  INTEGER NOT NULL REFERENCES tickets(id),
    creator_id INTEGER NOT NULL REFERENCES users(id),
    timestamp  TEXT    NOT NULL,
    message    TEXT    NOT NULL
);
