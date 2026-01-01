
# Todo Actix Web Server

A simple REST API server built with Actix Web in Rust, following along with a tutorial series. This project demonstrates basic web server setup with JSON response handling.

## 📋 Project Overview

This is a learning project that creates a basic web server using the Actix Web framework. It currently provides a simple status endpoint that returns a JSON response indicating the server status.

## 🚀 Features

- Simple HTTP server running on port 8080
- JSON serialization/deserialization with Serde
- Status endpoint (`GET /`) that returns server status
- Modular code structure with separate models module

## 🛠️ Prerequisites

- Rust programming language (latest stable version recommended)
- Cargo (Rust's package manager)

## 📦 Dependencies

- **actix-web**: Web framework for Rust
- **actix-rt**: Runtime for Actix Web
- **serde**: Serialization/deserialization framework
- **serde_json**: JSON serialization/deserialization

## 🏃‍♂️ Getting Started

### Installation

1. Clone or download this project
2. Navigate to the project directory
3. Build the project:
   ```bash
   cargo build
   ```

### Running the Server

Start the development server:
```bash
cargo run
```

The server will start at `http://127.0.0.1:8080`

### Testing the API

Once the server is running, you can test the status endpoint:

```bash
curl http://127.0.0.1:8080/
```

Expected response:
```json
{"status":"UP"}
```

Or visit `http://127.0.0.1:8080/` in your web browser.

## 📁 Project Structure

```
todo-actix/
├── src/
│   ├── main.rs          # Main application entry point
│   └── models.rs        # Data models and struct definitions
├── Cargo.toml           # Project dependencies and configuration
└── README.md            # This file
```

### Code Overview

#### main.rs
- Sets up the Actix Web server
- Defines the main application entry point
- Configures routes (currently only the root route)
- Handles server startup and binding

#### models.rs
- Contains the `Status` struct with Serde derive macros
- Enables JSON serialization/deserialization

## 🔧 Development

### Building for Release

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## 📚 Learning Resources

This project follows along with the YouTube tutorial series:
- **Title**: Rust Web Programming
- **Channel**: Referenced in your learning materials
- **Video**: https://www.youtube.com/watch?v=gQwA0g0NNSI&list=PLECOtlti4Psr4hXVX5GuSvLKp0-RZjz93

## 🚧 Next Steps

As you continue with the tutorial series, you'll likely add:
- Database integration
- CRUD operations for todo items
- More API endpoints
- Error handling
- Authentication
- Frontend integration

