-- Your SQL goes here
CREATE TABLE user_friend (
    idUser VARCHAR NOT NULL,
    idFriend VARCHAR NOT NULL,
    PRIMARY KEY (idUser, idFriend),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_friend
        FOREIGN KEY(idFriend)
            REFERENCES users(id)
            ON DELETE CASCADE
);