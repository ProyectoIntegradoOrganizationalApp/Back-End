-- Your SQL goes here
CREATE TABLE projects (
    id VARCHAR PRIMARY KEY NOT NULL,
    idUser VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    configuration JSON NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
);