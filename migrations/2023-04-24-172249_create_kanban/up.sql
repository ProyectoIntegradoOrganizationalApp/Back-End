-- Your SQL goes here
CREATE TABLE kanban (
    idApp VARCHAR NOT NULL,
    idProject VARCHAR NOT NULL,
    PRIMARY KEY(idApp, idProject),
    CONSTRAINT fk_app
        FOREIGN KEY(idApp)
            REFERENCES app(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(id)
            ON DELETE CASCADE
);