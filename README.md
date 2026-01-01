
# Rust Web Programming Learning Project

Welcome to my Rust web programming learning repository! This project contains my implementations and experiments while learning Rust web development.

## 📚 About This Repository

This repository documents my journey learning Rust web programming through hands-on projects and tutorials. Each project focuses on different aspects of building web applications with Rust, demonstrating progressive learning from basics to advanced features.

## 🏗️ Current Projects

### 📋 Todo Actix Web Server (Basic Version)
A simple REST API server built with Actix Web framework.

**Status**: ✅ Complete  
**Location**: `todo-actix/`

**Features**:
- Basic HTTP server setup
- JSON serialization with Serde
- Status endpoint
- Modular code structure

**Quick Start**:
```bash
cd todo-actix
cargo run
```

**Full Documentation**: [See detailed README](./todo-actix/README.md)

---

### 🔧 Todo Actix Web Server with Configuration
Enhanced version with environment-based configuration management.

**Status**: ✅ Complete  
**Location**: `todo-actix-config/`

**Features**:
- Environment variable configuration
- `.env` file support
- Configurable host and port settings

**Quick Start**:
```bash
cd todo-actix-config
cargo run
```

**Full Documentation**: [See detailed README](./todo-actix-config/README.md)

---

### 🗄️ Todo Actix Web Server with Database Integration
Full-featured version with PostgreSQL database integration.

**Status**: ✅ Complete  
**Location**: `todo-actix-db/`

**New Features**:
- PostgreSQL database connectivity
- Database connection pooling
- Todo list CRUD operations
- Advanced error handling

**Quick Start**:
```bash
cd todo-actix-db
# Set up database and environment variables first
cargo run
```

**Full Documentation**: [See detailed README](./todo-actix-db/README.md)

## 🎯 Learning Path

I'm following along with the YouTube tutorial series:
- **Series**: Rust Web Programming
- **Playlist**: [Watch on YouTube](https://www.youtube.com/watch?v=gQwA0g0NNSI&list=PLECOtlti4Psr4hXVX5GuSvLKp0-RZjz93)

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- Cargo package manager
- PostgreSQL (for database projects)

### Setup
1. Clone this repository
2. Navigate to the project you want to explore
3. Follow the specific README instructions for each project

## 📁 Repository Structure

```
rust-web-learning/
├── todo-actix/                 # Basic Actix Web server project
│   ├── src/
│   ├── Cargo.toml
│   └── README.md
├── todo-actix-config/          # Enhanced version with configuration
│   ├── src/
│   ├── Cargo.toml
│   ├── .env.example
│   └── README.md
├── todo-actix-db/              # Database-integrated version
│   ├── src/
│   ├── Cargo.toml
│   ├── .env.example
│   ├── database.sql
│   └── README.md
├── README.md                   # This main documentation file
└── (future projects will be added here)
```

## 🛠️ Technologies Used

- **Rust**: Systems programming language
- **Actix Web**: Powerful web framework
- **PostgreSQL**: Database management
- **Serde**: Serialization/deserialization
- **Config**: Configuration management
- **Dotenv**: Environment variable loading
- **Deadpool**: Database connection pooling
- **Tokio PG Mapper**: Row mapping utilities
- **Cargo**: Rust's package manager and build system

## 📖 Learning Goals

Through these projects, I aim to master:
- ✅ Basic web server setup in Rust
- ✅ Environment configuration management
- ✅ Database integration and connection pooling
- 🔲 REST API development
- 🔲 Authentication and authorization
- 🔲 Error handling patterns
- 🔲 Testing strategies
- 🔲 Deployment practices

## 📈 Progress Tracking

| Project | Status | Key Learnings |
|---------|--------|---------------|
| Todo Actix Web Server (Basic) | ✅ Complete | Basic server setup, JSON responses, routing |
| Todo Actix Web Server with Config | ✅ Complete | Environment variables, configuration management |
| Todo Actix Web Server with Database | ✅ Complete | PostgreSQL integration, connection pooling, ORM mapping |

## 🔄 Project Evolution

The projects showcase progressive learning:
1. **Basic Version** (`todo-actix/`): Fundamentals of Actix Web server setup
2. **Configuration Enhanced** (`todo-actix-config/`): Adding professional configuration management patterns
3. **Database Integrated** (`todo-actix-db/`): Full database integration with connection pooling and CRUD operations

## 🤝 Contributing

This is a personal learning repository, but I welcome:
- Suggestions for improvement
- Code review comments
- Additional learning resources
- Bug reports

## 📝 Notes

- Each project has its own detailed README with specific instructions
- Code comments explain my learning process and decisions
- I'm documenting challenges and solutions I encounter
- Projects build upon each other to demonstrate progressive learning

## 🔗 Useful Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Actix Web Documentation](https://actix.rs/docs/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Config Crate Documentation](https://docs.rs/config/latest/config/)
- [Deadpool Documentation](https://docs.rs/deadpool/latest/deadpool/)

---

*This repository showcases my progressive learning journey in Rust web development, with each project building upon the previous one to demonstrate real-world application development patterns.*
```
