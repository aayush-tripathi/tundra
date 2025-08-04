# ---- build ----
FROM rust:1.78-slim AS build
WORKDIR /app
COPY Cargo.toml .
COPY tundra/Cargo.toml tundra/Cargo.toml
COPY executor/Cargo.toml executor/Cargo.toml
RUN cargo fetch
COPY . .
RUN cargo build --release -p executor

# ---- runtime ----
FROM gcr.io/distroless/cc
COPY --from=build /app/target/release/executor /usr/local/bin/executor
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/executor"]
