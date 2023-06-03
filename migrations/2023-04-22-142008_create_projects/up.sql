-- Your SQL goes here
CREATE TABLE projects (
    idProject VARCHAR PRIMARY KEY NOT NULL,
    idUser VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    icon VARCHAR NOT NULL,
    state SMALLINT NOT NULL DEFAULT 1, -- 1. Public | 2. Private | 3. Friends
    created_at VARCHAR NOT NULL,
    updated_at VARCHAR NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
);