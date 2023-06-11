-- Your SQL goes here
CREATE TABLE project_user (
    idProject VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    idRole VARCHAR NOT NULL,
    joined_at TIMESTAMP NOT NULL,
    PRIMARY KEY (idProject, idUser),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_role
        FOREIGN KEY(idRole)
            REFERENCES "role"(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(idProject)
            ON DELETE CASCADE
);