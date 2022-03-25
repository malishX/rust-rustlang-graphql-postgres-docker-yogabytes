-- Your SQL goes here
CREATE TABLE member
(
    id           UUID PRIMARY KEY,
    team_id      UUID      NOT NULL,
    user_id      UUID      NOT NULL,
    name         VARCHAR   NOT NULL,
    identity_num VARCHAR   NOT NULL,
    role         VARCHAR   NOT NULL,
    assigned_at  TIMESTAMP NOT NULL,
    expired_at   TIMESTAMP,

    CONSTRAINT fk_team
        FOREIGN KEY (team_id)
            REFERENCES team (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
            REFERENCES "user" (id)
            ON DELETE CASCADE
)