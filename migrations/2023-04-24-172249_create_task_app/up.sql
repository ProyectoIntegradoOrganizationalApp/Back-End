-- Your SQL goes here
CREATE TYPE valid_task_app AS ENUM ('kanban', 'timeline');

CREATE TABLE task_app (
    idApp VARCHAR NOT NULL,
    idProject VARCHAR NOT NULL,
    app_type VARCHAR NOT NULL,
    PRIMARY KEY(idApp, idProject),
    CONSTRAINT fk_app
        FOREIGN KEY(idApp)
            REFERENCES app(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(idProject)
            ON DELETE CASCADE
);