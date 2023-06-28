FROM rust

# Build an empty dummy project to cache building dependencies
COPY Cargo.toml .
RUN mkdir src && touch src/main.rs
RUN echo "fn main() {}" >> /src/main.rs
RUN cargo build --release

# Copy the real project to the Docker image
RUN rm src/main.rs
COPY src /src
COPY .env .

# Build the program for release
RUN cargo build --release

# Run the binary
EXPOSE 8080
CMD ["./target/release/find-a-date-everyone-can-do"]
