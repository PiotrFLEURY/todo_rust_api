FROM rust AS builder

WORKDIR /sources

COPY . .

RUN cargo build --release --locked

FROM debian:12-slim

WORKDIR /app

COPY --from=builder /sources/target/release/todo_rust_api .
COPY --from=builder /sources/target/release/update_db .
COPY --from=builder /sources/db ./db

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user and ensure /app is owned by it
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

CMD ["sh", "-c", "./update_db && ./todo_rust_api"]