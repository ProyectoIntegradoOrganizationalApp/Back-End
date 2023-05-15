-- Your SQL goes here
CREATE TABLE projects (
    idProject VARCHAR PRIMARY KEY NOT NULL,
    idUser VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    updated_at VARCHAR NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
);