-- Your SQL goes here
CREATE TABLE "user"
(
    id    UUID PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    name  VARCHAR        NOT NULL
)