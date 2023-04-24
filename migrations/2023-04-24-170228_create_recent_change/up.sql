-- Your SQL goes here
CREATE TABLE recent_change (
    date DATE NOT NULL,
    idProject VARCHAR NOT NULL,
    backup JSON NOT NULL,
    PRIMARY KEY(date, idProject),
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(id)
);