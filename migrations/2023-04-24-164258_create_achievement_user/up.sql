-- Your SQL goes here
CREATE TABLE achievement_user (
    idAchievement VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    progress SMALLINT NOT NULL,
    completed BOOLEAN NOT NULL,
    PRIMARY KEY (idAchievement, idUser),
    CONSTRAINT fk_user
        FOREIGN KEY(idUser)
            REFERENCES users(id)
            ON DELETE CASCADE,
    CONSTRAINT fk_achievement
        FOREIGN KEY(idAchievement)
            REFERENCES achievement(id)
            ON DELETE CASCADE
);