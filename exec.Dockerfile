# Build Stage
FROM rust:1.79 AS build
WORKDIR /app

RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY Cargo.toml Cargo.lock ./
COPY tundra/Cargo.toml ./tundra/
COPY executor/Cargo.toml ./executor/
COPY tundra-cli/Cargo.toml ./tundra-cli/
RUN cargo fetch
COPY . .
RUN cargo build --release -p executor

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/executor /usr/local/bin/executor
EXPOSE 8080
CMD ["executor"]