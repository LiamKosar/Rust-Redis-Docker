FROM rust:1.86

WORKDIR /app

# Copy main project files
COPY Cargo.toml .
COPY src/ ./src/

# Explicitly copy the macro directory
COPY celerylib/src ./celerylib/src/
COPY celerylib/Cargo.toml ./celerylib
# Verify the structure
# RUN ls -la
# RUN ls -la celery-macros/src/cargo.toml

RUN cargo build --release

# Use the absolute path in CMD
CMD ["/app/target/release/rust-app"]