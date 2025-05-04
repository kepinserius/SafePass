# SafePass API

A secure password management API built with Rust and Actix-web for storing and retrieving encrypted passwords.

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange" alt="Rust Version">
  <img src="https://img.shields.io/badge/License-MIT-blue" alt="License">
  <img src="https://img.shields.io/badge/API-RESTful-green" alt="API Type">
</p>

## 📋 Overview

SafePass API is a secure backend service that allows users to store sensitive password data with military-grade encryption. The API provides comprehensive user authentication, password management, and security features to ensure your data remains protected.

## ✨ Features

- **User Management**

  - Registration with email verification
  - Secure login with rate limiting
  - Profile management
  - Password reset capability

- **Password Security**

  - AES-256 encryption for all stored passwords
  - Unique encryption keys per user
  - Secure notes storage

- **API Security**

  - JWT-based authentication with automatic refreshing
  - Rate limiting to prevent brute force attacks
  - Input validation and sanitization
  - CORS protection

- **Database**
  - PostgreSQL for reliable data storage
  - Diesel ORM for type-safe database operations
  - Migrations for easy versioning

## 🔧 Technology Stack

- **Backend**: Rust 1.75+
- **Web Framework**: Actix-web 4.x
- **Database**: PostgreSQL 13+
- **ORM**: Diesel with r2d2 connection pooling
- **Authentication**: JWT (JSON Web Tokens)
- **Encryption**: AES-256-GCM
- **Password Hashing**: Bcrypt with salting
- **API Documentation**: OpenAPI/Swagger

## 🚀 Getting Started

### Prerequisites

- Rust toolchain (1.75 or newer)
- Cargo package manager
- PostgreSQL 13+ database
- OpenSSL development libraries

### Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/kepinserius/safepass.git
   cd safepass
   ```

2. **Set Up PostgreSQL Database**

   ```bash
   createdb safepass
   psql -d safepass -f migrations/up.sql
   ```

3. **Configure Environment Variables**
   Create a `.env` file in the project root:

   ```
   DATABASE_URL=postgres://username:password@localhost/safepass
   HOST=127.0.0.1
   PORT=8080
   JWT_SECRET=your_super_secret_jwt_key_replace_in_production
   ENCRYPTION_KEY=your_super_secret_encryption_key_32chars
   ```

4. **Generate Strong Secret Keys** (Recommended)

   ```bash
   # Generate JWT secret
   openssl rand -base64 32

   # Generate encryption key
   openssl rand -base64 32
   ```

5. **Build the Project**

   ```bash
   cargo build --release
   ```

6. **Run the API**
   ```bash
   cargo run --release
   ```

The API will be available at `http://127.0.0.1:8080` (or the host/port specified in your `.env` file).

## 📡 API Documentation

### User Management Endpoints

| Endpoint             | Method | Description         | Request Body                                                                | Authentication |
| -------------------- | ------ | ------------------- | --------------------------------------------------------------------------- | -------------- |
| `/api/user/register` | POST   | Register a new user | `{"username": "user", "email": "user@example.com", "password": "password"}` | None           |
| `/api/user/login`    | POST   | Authenticate user   | `{"email": "user@example.com", "password": "password"}`                     | None           |
| `/api/user/profile`  | GET    | Get user profile    | None                                                                        | JWT            |
| `/api/user/profile`  | PUT    | Update user profile | `{"username": "newname"}`                                                   | JWT            |

### Password Management Endpoints

| Endpoint              | Method | Description             | Request Body                                                                                                                           | Authentication |
| --------------------- | ------ | ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| `/api/passwords`      | GET    | Get all passwords       | None                                                                                                                                   | JWT            |
| `/api/passwords`      | POST   | Create a new password   | `{"site_name": "Example", "site_url": "https://example.com", "username": "user", "password": "securepass", "notes": "Optional notes"}` | JWT            |
| `/api/passwords/{id}` | GET    | Get a specific password | None                                                                                                                                   | JWT            |
| `/api/passwords/{id}` | PUT    | Update a password       | `{"site_name": "Updated", "password": "newsecurepass"}`                                                                                | JWT            |
| `/api/passwords/{id}` | DELETE | Delete a password       | None                                                                                                                                   | JWT            |

## 🔒 Security Implementation

- **Password Storage**: All passwords are encrypted with AES-256-GCM before storage
- **User Passwords**: Hashed using Bcrypt with work factor 12
- **Authentication**: Stateless JWT tokens with automatic expiration
- **Rate Limiting**: IP-based rate limiting to prevent brute force attacks
- **CSRF Protection**: Token-based Cross-Site Request Forgery protection

## ⚙️ Configuration Options

| Environment Variable | Description                  | Default                                           |
| -------------------- | ---------------------------- | ------------------------------------------------- |
| `DATABASE_URL`       | PostgreSQL connection string | `postgres://postgres:postgres@localhost/safepass` |
| `HOST`               | API host address             | `127.0.0.1`                                       |
| `PORT`               | API port                     | `8080`                                            |
| `JWT_SECRET`         | Secret key for JWT signing   | None (Required)                                   |
| `ENCRYPTION_KEY`     | Key for password encryption  | None (Required)                                   |
| `JWT_EXPIRY`         | JWT token expiry in seconds  | `86400` (24 hours)                                |
| `RATE_LIMIT`         | Max requests per minute      | `60`                                              |

## 📝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [Actix-web](https://actix.rs/)
- [Diesel](https://diesel.rs/)
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- [Argon2](https://github.com/P-H-C/phc-winner-argon2)
