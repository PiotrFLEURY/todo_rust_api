# Rust Todo Api using Clean Architecture

This is a simple Todo API implemented in Rust using the Clean Architecture principles. The project is structured into different layers, including the domain layer, application layer, and infrastructure layer.

## Project Structure

- `src/data`: Contains the data models and database interactions.
- `src/domain`: Contains the core business logic and entities.
- `src/presentation`: Contains the API handlers and routes.
- `src/main.rs`: The entry point of the application.

## Requirements

- Rust
- A database (e.g., PostgreSQL)

## Getting Started

### Configuration

Create a `.env` file in the root of the project with the following content:

```env
DATABASE_URL=postgres://user:user_password@address/db_name
BIND_ADDRESS=127.0.0.1
PORT=8080
```

### Running the Application

1. Install the dependencies:

```bash
cargo build
```

2. Run the application:

```bash
cargo run
```

The API will be available at `http://localhost:8080` (replace with your actual bind address and port if different).

## API Endpoints

- `POST /todos`: Create a new todo item.
- `GET /todos`: Retrieve all todo items.
- `GET /todos/{id}`: Retrieve a specific todo item by ID.
- `PUT /todos/{id}`: Update a specific todo item by ID.
- `DELETE /todos/{id}`: Delete a specific todo item by ID.

## Init Database

To initialize the database, you can use the files in the `db` directory.

See `db/tables.sql` for the SQL commands to create the necessary tables.

## Tools

Some test http requests can be found in the `tools` directory, which can be used to interact with the API endpoints for testing purposes.