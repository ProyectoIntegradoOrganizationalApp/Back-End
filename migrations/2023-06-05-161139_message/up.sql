-- Your SQL goes here
CREATE TABLE message (
    id VARCHAR NOT NULL,
    idSender VARCHAR NOT NULL,
    idFriend VARCHAR NULL,
    idGroup VARCHAR NULL,
    content TEXT NOT NULL,
    sent_at TIMESTAMP NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_sender
        FOREIGN KEY(idSender)
            REFERENCES users(id),
    CONSTRAINT fk_friend
        FOREIGN KEY(idFriend)
            REFERENCES users(id),
    CONSTRAINT fk_group
        FOREIGN KEY(idGroup)
            REFERENCES groups(id)
);