FROM rust:1.87-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY constants.is .
COPY is/ is/

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/crates/server/static/ /app/crates/server/static/
COPY --from=builder /app/is/ /app/is/

EXPOSE 8080
CMD ["./server"]
