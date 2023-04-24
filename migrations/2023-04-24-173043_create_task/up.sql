-- Your SQL goes here
CREATE TABLE task (
    id VARCHAR NOT NULL,
    idColumn VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description VARCHAR,
    github VARCHAR,
    PRIMARY KEY(id),
    CONSTRAINT fk_column
        FOREIGN KEY(idColumn)
            REFERENCES columna(id)
            ON DELETE CASCADE
);