-- Your SQL goes here
CREATE TYPE valid_roles AS ENUM ('administrator', 'editor', 'reader');

CREATE TABLE role (
    id VARCHAR PRIMARY KEY NOT NULL,
    name valid_roles UNIQUE
);

INSERT INTO "role" VALUES
('1', 'administrator'),
('2', 'editor'),
('3', 'reader')