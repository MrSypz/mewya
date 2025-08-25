# {{ project-name }}

A lightweight Rust web API built with Axum.

## Quick Start

1. Clone this repository
2. Copy `.env.example` to `.env` and configure
3. Run the development server:

\`\`\`bash
cargo run
\`\`\`

The server will start at `http://localhost:3000`

## API Endpoints

- `GET /` - Hello world
- `GET /health` - Health check

## Development

\`\`\`bash
# Run with auto-reload (install cargo-watch first)
cargo install cargo-watch
cargo watch -x run


# Build for production
cargo build --release
\`\`\`

## Optional Features

Uncomment dependencies in `Cargo.toml` as needed:

- **Logging**: `tracing` + `tracing-subscriber`
- **CORS & Middleware**: `tower-http`
- **Database**: `sqlx`
- **Environment**: `dotenvy`
- **Time**: `chrono`
- **UUID**: `uuid`

Uncomment `all the file` and setting `database url`