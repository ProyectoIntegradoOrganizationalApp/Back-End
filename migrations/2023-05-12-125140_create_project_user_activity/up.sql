-- Your SQL goes here
CREATE TABLE project_user_activity (
    idUser VARCHAR NOT NULL,
    idProject VARCHAR NOT NULL,
    "date" VARCHAR NOT NULL,
    commits SMALLINT NOT NULL,
    PRIMARY KEY (idUser, idProject, "date"),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(id)
            ON DELETE CASCADE
);