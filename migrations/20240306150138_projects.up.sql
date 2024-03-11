-- Add migration script here
CREATE TABLE IF NOT EXISTS projects
(
    id          BIGINT  UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    name        TEXT    NOT NULL DEFAULT "New Project",
    category    TEXT    NOT NULL,
    position    BIGINT  UNSIGNED NOT NULL,
    status      BIGINT  UNSIGNED NOT NULL DEFAULT 0,
    notes       TEXT    NOT NULL DEFAULT "",
    creation_date DATE NOT NULL DEFAULT CURRENT_DATE,
    start_date  DATE NOT NULL DEFAULT CURRENT_DATE,
    completion_date DATE NOT NULL DEFAULT CURRENT_DATE
);
