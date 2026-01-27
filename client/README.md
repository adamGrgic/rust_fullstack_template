# Client

Leptos-based web client for the Atom Platform todo management system.

## Features

- Modern, responsive UI built with Leptos
- Real-time CRUD operations
- Status management for todos
- Client-side rendering (CSR)

## Prerequisites

- Rust and Cargo
- Trunk (for building and serving): `cargo install trunk`
- wasm32 target: `rustup target add wasm32-unknown-unknown`

## Development

1. Ensure the API server is running on `http://localhost:8080`
2. Start the development server:

```bash
trunk serve
```

3. Open your browser to `http://localhost:8081`

## Building for Production

```bash
trunk build --release
```

The built files will be in the `dist/` directory.

## Configuration

The API base URL is configured in `src/api.rs`. Update the `API_BASE` constant to point to your API server.

