-- This migration file is used to create the `todos` table.
-- 

CREATE TABLE IF NOT EXISTS todos
(
    id          BIGSERIAL PRIMARY KEY,
    description TEXT    NOT NULL,
    done        BOOLEAN NOT NULL DEFAULT FALSE
);