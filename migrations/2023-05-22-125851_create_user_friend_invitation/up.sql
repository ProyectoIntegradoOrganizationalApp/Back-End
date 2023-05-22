-- Your SQL goes here
CREATE TABLE user_friend_invitation (
    idGuest VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    message VARCHAR NOT NULL,
    PRIMARY KEY (idGuest, idUser),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_guest
        FOREIGN KEY(idGuest)
            REFERENCES users(id)
            ON DELETE CASCADE
);