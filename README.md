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

## Tests

To run the tests, use the following command:

```bash
cargo test
```

### Unit Tests

Unit tests are located in the same files as the code they test, using the `#[cfg(test)]` attribute.

### Integration Tests

Integration tests are located in the `tests` directory and can be run using the same `cargo test` command.

> Note: Integration tests use `testcontainers` to create a temporary database instance for testing purposes. Ensure you have Docker installed and running on your machine to use this feature.

**End to End**

Integration tests uses `tower` to make http requests to the API endpoints and verify the responses. The tests are designed to cover the entire flow of creating, retrieving, updating, and deleting todo items.

Since the tests use http requests to test the API endpoints through all the layers of the application, they are considered end-to-end tests.

## API Documentation

The API documentation is generated using `utoipa` and can be accessed at `http://localhost:8080/swagger-ui` after running the application. This provides an interactive interface to explore and test the API endpoints.

See `src/presentation/api.rs` for the configuration of the API documentation.

See `src/presentation/routes.rs` for the routes configuration, which includes the integration of the Swagger UI for API documentation.

See `src/presentation/handlers.rs` for the implementation of the API handlers, which are responsible for processing the incoming requests and returning the appropriate responses.

## Docker

A Dockerfile is included in the project for containerizing the application. You can build and run the Docker image using the following commands:

```bash
# Build the Docker image
docker build -t todo_rust_api:latest .
# Run the Docker container
docker run -p 8080:8080 --env-file .env todo_rust_api:latest
```

This will build the Docker image and run the container, exposing the API on port 8080. Make sure to adjust the port and environment variables as needed.

### Docker compose

> Env variables for the Docker container are defined in the `docker.env` file, which is used by the `docker-compose.yml` file to set up the environment for the application and its dependencies.

A `docker-compose.yml` file is included for easier management of the application and its dependencies (e.g., database). You can start the application using Docker Compose with the following command:

```bash
docker compose up
```

This will start both the application and the database services defined in the `docker-compose.yml` file. The API will be accessible at `http://localhost:8080`.

## CI/CD

### Rust CI

A GitHub Actions workflow is set up to run the tests on every push to the main branch. The workflow is defined in `.github/workflows/rust.yml`.

### Docker Image

A GitHub Actions workflow is set up to build and push the Docker image to GitHub Container Registry (GHCR) on every push to the main branch. The workflow is defined in `.github/workflows/docker.yml`.

