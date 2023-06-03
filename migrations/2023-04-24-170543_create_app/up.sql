-- Your SQL goes here
CREATE TABLE app (
    id VARCHAR NOT NULL,
    idproject VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    photo VARCHAR NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(idProject)
);