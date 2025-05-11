FROM rust:1.86

WORKDIR /app

# Copy main project files
COPY Cargo.toml .
COPY src/ ./src/

# Explicitly copy the macro directory
COPY celery-macros/src ./celery-macros/src/
COPY celery-macros/Cargo.toml ./celery-macros
# Verify the structure
# RUN ls -la
# RUN ls -la celery-macros/src/cargo.toml

RUN cargo build --release

# Use the absolute path in CMD
CMD ["/app/target/release/rust-app"]