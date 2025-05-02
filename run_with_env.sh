#!/bin/bash

# Set environment variables
export DATABASE_URL="postgres://postgres@localhost:5432/safepass"
export JWT_SECRET="super_secret_jwt_key_for_token_generation_and_validation"
export ENCRYPTION_KEY="32_character_key_for_password_encryption_security"
export RUST_LOG=debug

# Run database migrations
echo "Running database migrations..."
diesel migration run

# Run the application
echo "Starting application..."
cargo run 