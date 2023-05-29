-- Your SQL goes here
CREATE TABLE role (
    id VARCHAR PRIMARY KEY NOT NULL,
    name VARCHAR UNIQUE NOT NULL
);

INSERT INTO "role" VALUES
('1', 'administrator'),
('2', 'editor'),
('3', 'reader')