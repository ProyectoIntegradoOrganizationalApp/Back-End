-- Your SQL goes here
CREATE TABLE group_user (
    idGroup VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    joined_at TIMESTAMP NOT NULL,
    PRIMARY KEY(idGroup, idUser),
    CONSTRAINT fk_group
        FOREIGN KEY(idGroup)
            REFERENCES groups(id),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
);