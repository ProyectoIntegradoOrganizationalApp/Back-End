-- Your SQL goes here
CREATE TABLE app (
    id VARCHAR NOT NULL,
    idproject VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    photo VARCHAR NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_project
        FOREIGN KEY(idproject)
            REFERENCES projects(idProject),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
);