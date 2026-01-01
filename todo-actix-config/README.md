
# Todo Actix Web Server with Database Integration

A full-featured REST API server built with Actix Web and PostgreSQL, featuring database integration and connection pooling.

## 📋 Project Overview

This project extends the configuration-enhanced version by adding PostgreSQL database support. It demonstrates real-world web application patterns including database connectivity, connection pooling, and CRUD operations.

## 🚀 Features

- **PostgreSQL Database Integration** with connection pooling
- **Database-driven Todo Lists API** with GET operations
- **Environment-based Configuration** for database settings
- **Connection Pool Management** using Deadpool
- **Row Mapping** with Tokio PG Mapper
- **Modular Code Structure** with separate handlers and database modules

## 🛠️ Prerequisites

- Rust (latest stable version)
- Cargo package manager
- PostgreSQL database server
- Basic knowledge of SQL

## 📦 Dependencies

- **actix-web**: Web framework for Rust
- **deadpool-postgres**: PostgreSQL connection pooling
- **tokio-postgres**: Async PostgreSQL client
- **tokio-pg-mapper**: Row to struct mapping
- **config**: Configuration management
- **dotenv**: Environment variable loading
- **serde**: Serialization/deserialization

## 🗄️ Database Setup

### 1. Create Database and User

```sql
CREATE DATABASE todo_app;
CREATE USER todo_user WITH PASSWORD 'yourpassword';
GRANT ALL PRIVILEGES ON DATABASE todo_app TO todo_user;
```

### 2. Initialize Schema

Run the provided SQL script:
```bash
psql -d todo_app -f database.sql
```

### Database Schema

```sql
todo_lists (id, title)
todo_items (id, title, checked, list_id)
```

## 🏃‍♂️ Getting Started

### Installation

1. Navigate to the project directory
2. Set up environment variables (see Configuration section)
3. Build the project:
```bash
cargo build
```

### Configuration

Create a `.env` file:
```env
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
PG.HOST=localhost
PG.PORT=5432
PG.USER=todo_user
PG.PASSWORD=yourpassword
PG.DBNAME=todo_app
PG.POOL.MAX_SIZE=16
```

### Running the Server

```bash
cargo run
```

Server will start at: `http://127.0.0.1:8080`

## 🌐 API Endpoints

### GET `/`
**Health Check Endpoint**
```bash
curl http://localhost:8080/
```
Response:
```json
{"status":"UP"}
```

### GET `/todo_lists`
**Retrieve All Todo Lists**
```bash
curl http://localhost:8080/todo_lists
```
Response:
```json
[
  {"id": 1, "title": "List 1"},
  {"id": 2, "title": "List 2"}
]
```

## 📁 Project Structure

```
todo-actix-db/
├── src/
│   ├── main.rs          # Application entry point
│   ├── config.rs        # Configuration management
│   ├── models.rs        # Data structures
│   ├── handler.rs       # HTTP request handlers
│   └── db.rs           # Database operations
├── database.sql         # Database schema and sample data
├── .env.example         # Example environment variables
├── Cargo.toml          # Dependencies and configuration
└── README.md           # This file
```

### Code Overview

#### main.rs
- Initializes configuration and database pool
- Sets up Actix Web server with route configuration
- Manages application lifecycle

#### config.rs
- Defines configuration structures
- Loads settings from environment variables
- Configures database connection pooling

#### models.rs
- Data models with Serde and PostgresMapper derives
- TodoList and Status structs

#### handler.rs
- HTTP request handlers
- Database operation orchestration
- Error handling and response formatting

#### db.rs
- Database query functions
- Row to struct mapping
- SQL operation implementations

## ⚙️ Configuration Options

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SERVER_HOST` | `127.0.0.1` | Server host address |
| `SERVER_PORT` | `8080` | Server port |
| `PG.HOST` | `localhost` | PostgreSQL host |
| `PG.PORT` | `5432` | PostgreSQL port |
| `PG.USER` | - | Database user (required) |
| `PG.PASSWORD` | - | Database password (required) |
| `PG.DBNAME` | - | Database name (required) |
| `PG.POOL.MAX_SIZE` | `16` | Connection pool size |

## 🔧 Development

### Building and Testing

```bash
# Build project
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Database Operations

The project includes sample data:
- 2 todo lists ("List 1", "List 2")
- 2 todo items in "List 1" ("Connect to database", "Do queries")

## 🐛 Troubleshooting

### Common Issues

**Database Connection Errors:**
- Verify PostgreSQL is running
- Check connection parameters in `.env`
- Ensure database and user exist

**Compilation Errors:**
- Check all dependencies in Cargo.toml
- Verify Rust version compatibility

**Runtime Errors:**
- Check database schema matches models
- Verify environment variables are set

## 🚧 Next Steps

Potential enhancements:
- Complete CRUD operations for todo lists and items
- Add authentication and authorization
- Implement request validation
- Add comprehensive error handling
- Write unit and integration tests
- Add logging and monitoring

## 📚 Learning Outcomes

This project demonstrates:
- Database connection pooling patterns
- Async/await database operations
- Row to struct mapping techniques
- Modular application architecture
- Environment-based configuration
- Error handling in web applications
