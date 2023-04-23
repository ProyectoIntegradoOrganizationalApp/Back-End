-- Your SQL goes here

-- CREATE TABLE users (
--     id VARCHAR PRIMARY KEY NOT NULL,
--     email VARCHAR NOT NULL UNIQUE,
--     password VARCHAR NOT NULL,
--     name VARCHAR NOT NULL,
--     lastname VARCHAR,
--     phone VARCHAR,
--     created_at DATE NOT NULL,
--     updated_at DATE NOT NULL
-- );

CREATE TABLE users (
    id VARCHAR PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL
);