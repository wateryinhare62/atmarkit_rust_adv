-- Add up migration script here
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    posted DATETIME NOT NULL,
    sender TEXT NOT NULL,
    content TEXT NOT NULL
);