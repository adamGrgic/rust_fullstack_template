# Platform API

REST API for the Atom Platform todo management system.

## Features

- CRUD operations for todos
- PostgreSQL database with sqlx
- Automatic migrations on startup
- RESTful endpoints

## Setup

1. Copy `.env.example` to `.env` and configure your database connection
2. Ensure PostgreSQL is running
3. Run the API: `cargo run`

The migrations will run automatically on startup.

## API Endpoints

- `GET /health` - Health check
- `GET /api/todos` - List all todos
- `GET /api/todos/:id` - Get a specific todo
- `POST /api/todos` - Create a new todo
- `PUT /api/todos/:id` - Update a todo
- `DELETE /api/todos/:id` - Delete a todo

## Example Requests

### Create a todo
```bash
curl -X POST http://localhost:8080/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "My first todo", "description": "This is a test"}'
```

### List todos
```bash
curl http://localhost:8080/api/todos
```

### Update a todo
```bash
curl -X PUT http://localhost:8080/api/todos/{id} \
  -H "Content-Type: application/json" \
  -d '{"status": "completed"}'
```

### Delete a todo
```bash
curl -X DELETE http://localhost:8080/api/todos/{id}
```

