-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    subject VARCHAR NOT NULL,
    username VARCHAR NOT NULL
)
