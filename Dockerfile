# Build stage
FROM rust:1.78 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release
RUN strip target/release/app

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libncursesw6 \
 && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/app .

ENTRYPOINT ["./app"]

