-- Your SQL goes here
CREATE TABLE project_user (
    id VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    idRole VARCHAR NOT NULL,
    PRIMARY KEY (id, idUser),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_role
        FOREIGN KEY(idRole)
            REFERENCES "role"(id)
            ON DELETE CASCADE
);