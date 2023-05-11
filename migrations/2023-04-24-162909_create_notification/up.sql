CREATE TABLE notification (
    id VARCHAR PRIMARY KEY NOT NULL,
    idUser VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    state BOOLEAN NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
);