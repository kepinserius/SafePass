-- Create extension for UUID support
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Create passwords table
CREATE TABLE IF NOT EXISTS passwords (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    site_name VARCHAR(255) NOT NULL,
    site_url VARCHAR(255),
    username VARCHAR(255) NOT NULL,
    encrypted_password TEXT NOT NULL,
    encryption_iv TEXT NOT NULL,
    notes TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_passwords_user_id ON passwords (user_id);
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email); 