-- Add migration script here
CREATE TABLE IF NOT EXISTS projects
(
    id          BIGINT  UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    catagory    TEXT    NOT NULL,
    position    BIGINT  UNSIGNED NOT NULL,
    status      BIGINT  UNSIGNED NOT NULL DEFAULT 0,
    notes       TEXT,
    creation_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    start_time  TIMESTAMP,
    completion_time TIMESTAMP
);
