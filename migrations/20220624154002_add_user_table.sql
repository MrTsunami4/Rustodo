-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
	user TEXT NOT NULL,
    password TEXT NOT NULL
)