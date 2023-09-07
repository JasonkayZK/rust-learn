-- Your SQL goes here
create table if not exists t_person
(
    uid         TEXT    NOT NULL PRIMARY KEY,
    name        TEXT    NOT NULL DEFAULT '',
    age         INTEGER NOT NULL DEFAULT 0,
    info        TEXT    NOT NULL DEFAULT '',
    gender      INTEGER NOT NULL DEFAULT 0,
    create_time INTEGER NOT NULL DEFAULT 0
);
