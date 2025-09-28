-- Add migration script here
-- migrations/XXXXXX_create_credentials_table.sql
CREATE TABLE IF NOT EXISTS credentials (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_credentials_email ON credentials(email);