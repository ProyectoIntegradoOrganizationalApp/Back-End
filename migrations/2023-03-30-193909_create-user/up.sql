-- Your SQL goes here

CREATE TABLE users (
    id VARCHAR PRIMARY KEY NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    updated_at VARCHAR NOT NULL,
    level SMALLINT NOT NULL
);