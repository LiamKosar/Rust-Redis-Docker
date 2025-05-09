FROM rust:1.86

WORKDIR /app

COPY . .

RUN cargo build --release

# Use the absolute path in CMD
CMD ["/app/target/release/rust-worker"]