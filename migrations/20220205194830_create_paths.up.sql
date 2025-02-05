-- Add up migration script here
CREATE TABLE IF NOT EXISTS `paths` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `path` TEXT NOT NULL UNIQUE,
    `score` INTEGER NOT NULL
);
