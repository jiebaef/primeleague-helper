-- Add migration script here

CREATE SCHEMA primeleague;

CREATE TABLE primeleague.cached_responses (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    data TEXT NOT NULL
);
