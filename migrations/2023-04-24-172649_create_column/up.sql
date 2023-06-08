-- Your SQL goes here
CREATE TABLE columna (
    id VARCHAR NOT NULL,
    idBoard VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    updated_at VARCHAR NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_board
        FOREIGN KEY(idBoard)
            REFERENCES board(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE
);