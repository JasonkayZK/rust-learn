CREATE TABLE person
(
    id   INTEGER PRIMARY KEY,
    name TEXT    NOT NULL,
    age  INTEGER NOT NULL DEFAULT 0,
    data BLOB
);
