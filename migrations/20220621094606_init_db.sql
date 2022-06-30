-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
	id TEXT NOT NULL,
	content TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
)