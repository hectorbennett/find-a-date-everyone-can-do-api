# 1. This tells docker to use the Rust official image
FROM rust

# 2. Copy the files in your machine to the Docker image
COPY src /src
COPY Cargo.toml .

# Build your program for release
RUN cargo build --release

# Run the binary
EXPOSE 8080
CMD ["./target/release/find-a-date-everyone-can-do"]
