# exec.Dockerfile
FROM rust:1.75 AS build
WORKDIR /app
COPY . .
RUN cargo fetch
RUN cargo build --release -p executor

# ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/executor /usr/local/bin/executor
EXPOSE 8080
CMD ["executor"]
