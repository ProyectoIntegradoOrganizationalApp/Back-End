-- Your SQL goes here
CREATE TYPE valid_states AS ENUM ('checked', 'unchecked');

CREATE TABLE notification (
    id VARCHAR PRIMARY KEY NOT NULL,
    idUser VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    state valid_states,
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
);