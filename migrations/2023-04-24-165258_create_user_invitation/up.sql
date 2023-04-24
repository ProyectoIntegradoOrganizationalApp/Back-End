-- Your SQL goes here
CREATE TABLE user_invitation (
    idProject VARCHAR NOT NULL,
    idGuest VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    PRIMARY KEY (idProject, idGuest, idUser),
    CONSTRAINT fk_project
        FOREIGN KEY(idProject)
            REFERENCES projects(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_guest
        FOREIGN KEY(idGuest)
            REFERENCES users(id)
            ON DELETE CASCADE
);