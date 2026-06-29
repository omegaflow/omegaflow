FROM rust:1.96-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/omegaflow /app/server
COPY static/ static/
COPY φ/ φ/
EXPOSE 3571
CMD ["./server"]

