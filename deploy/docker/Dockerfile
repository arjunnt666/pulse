FROM rust:1.80-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p pulse-cli || true

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/pulse /usr/local/bin/pulse || true
CMD ["pulse", "version"]
