-- Your SQL goes here
CREATE TABLE task (
    id VARCHAR NOT NULL,
    idColumn VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    idProject VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description VARCHAR,
    state SMALLINT NOT NULL DEFAULT 0,
    completed_at TIMESTAMP NULL,
    created_at VARCHAR NOT NULL,
    updated_at VARCHAR NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_column
        FOREIGN KEY(idColumn)
            REFERENCES columna(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(idProject)
            ON DELETE CASCADE
);