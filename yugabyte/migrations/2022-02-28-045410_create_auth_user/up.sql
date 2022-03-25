-- Your SQL goes here
-- I created this table separately for security purposes. and I prefer to put this table
-- in another database to prevent knowing the password if the database has been hacked.

CREATE TABLE auth_user
(
    id       UUID PRIMARY KEY,
    email    VARCHAR UNIQUE NOT NULL,
    password VARCHAR        NOT NULL
)