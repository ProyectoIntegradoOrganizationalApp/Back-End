-- Your SQL goes here
CREATE TYPE valid_roles AS ENUM ('administrator', 'boss', 'normal');

CREATE TABLE role (
    id VARCHAR PRIMARY KEY NOT NULL,
    role valid_roles UNIQUE
);