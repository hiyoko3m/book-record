-- Add up migration script here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL
)
