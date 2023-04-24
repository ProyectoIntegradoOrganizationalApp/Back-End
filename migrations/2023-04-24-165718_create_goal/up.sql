-- Your SQL goes here
CREATE TABLE goal (
    id VARCHAR PRIMARY KEY NOT NULL,
    idProject VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR,
    completed SMALLINT NOT NULL,
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(id)
);