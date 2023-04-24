-- Your SQL goes here
CREATE TABLE columna (
    id VARCHAR NOT NULL,
    idBoard VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_board
        FOREIGN KEY(idBoard)
            REFERENCES board(id)
            ON DELETE CASCADE
);