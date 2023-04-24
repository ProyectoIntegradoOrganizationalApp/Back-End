-- Your SQL goes here
CREATE TABLE app (
    id VARCHAR NOT NULL,
    idProject VARCHAR NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(id)
);