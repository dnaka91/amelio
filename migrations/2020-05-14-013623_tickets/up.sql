CREATE TABLE tickets
(
    id          INTEGER NOT NULL PRIMARY KEY,
    type        TEXT    NOT NULL,
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL,
    category    TEXT    NOT NULL,
    priority    TEXT    NOT NULL,
    status      TEXT    NOT NULL DEFAULT 'open',
    course_id   INTEGER NOT NULL REFERENCES courses(id),
    creator_id  INTEGER NOT NULL REFERENCES users(id),
    CHECK (type IN (
        'course-book',
        'reading-list',
        'interactive-book',
        'practice-exam',
        'practice-exam-solution',
        'vodcast',
        'podcast',
        'presentation',
        'live-tutorial-recording',
        'online-test'
    )),
    CHECK (category IN (
        'editorial',
        'content',
        'improvement',
        'addition'
    )),
    CHECK (priority IN (
        'critical',
        'high',
        'medium',
        'low'
    )),
    CHECK (status IN (
        'open',
        'in-progress',
        'accepted',
        'refused',
        'completed'
    ))
);

CREATE TABLE medium_texts (
    ticket_id INTEGER NOT NULL PRIMARY KEY REFERENCES tickets(id),
    page      INTEGER NOT NULL,
    line      INTEGER NOT NULL
);

CREATE TABLE medium_recordings (
    ticket_id INTEGER NOT NULL PRIMARY KEY REFERENCES tickets(id),
    time      TEXT    NOT NULL
);

CREATE TABLE medium_interactives (
    ticket_id INTEGER NOT NULL PRIMARY KEY REFERENCES tickets(id),
    url       TEXT    NOT NULL
);

CREATE TABLE medium_questionaires (
    ticket_id INTEGER NOT NULL PRIMARY KEY REFERENCES tickets(id),
    question  INTEGER NOT NULL,
    answer    TEXT    NOT NULL
);
