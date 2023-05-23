-- Your SQL goes here
CREATE TABLE achievement_user (
    idAchievement VARCHAR NOT NULL,
    idUser VARCHAR NOT NULL,
    progress INT NOT NULL,
    percentage NUMERIC(10, 2) NOT NULL,
    current_state INT NOT NULL,
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