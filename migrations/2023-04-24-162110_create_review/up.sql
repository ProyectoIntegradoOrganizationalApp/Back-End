-- Your SQL goes here
CREATE TABLE review (
    id VARCHAR PRIMARY KEY NOT NULL,
    idUser VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    rating NUMERIC (2,1),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
);