-- Your SQL goes here
CREATE TABLE board (
    id VARCHAR NOT NULL,
    idApp VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    photo VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL,
    updated_at VARCHAR NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_app
        FOREIGN KEY(idApp)
            REFERENCES app(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE
);