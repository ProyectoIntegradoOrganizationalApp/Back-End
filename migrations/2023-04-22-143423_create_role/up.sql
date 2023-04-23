-- Your SQL goes here
CREATE TYPE valid_roles AS ENUM ('administrator', 'boss', 'normal');

CREATE TABLE role (
    id VARCHAR PRIMARY KEY NOT NULL,
    name valid_roles UNIQUE
);