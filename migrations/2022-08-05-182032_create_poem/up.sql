-- Your SQL goes here

CREATE TABLE poems (
    id VARCHAR NOT NULL PRIMARY KEY,
    subject_id VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    like INTEGER NOT NULL DEFAULT 0
);