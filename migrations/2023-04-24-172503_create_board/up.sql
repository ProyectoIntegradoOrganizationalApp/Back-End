-- Your SQL goes here
CREATE TABLE board (
    id VARCHAR NOT NULL,
    idApp VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_app
        FOREIGN KEY(idApp)
            REFERENCES app(id)
            ON DELETE CASCADE
);