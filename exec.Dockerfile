
FROM rust:latest AS build
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY tundra/Cargo.toml ./tundra/
COPY executor/Cargo.toml ./executor/
COPY tundra-cli/Cargo.toml ./tundra-cli/

RUN mkdir -p tundra/src executor/src tundra-cli/src && \
    echo "fn main() {}" > tundra/src/main.rs && \
    echo "fn main() {}" > executor/src/main.rs && \
    echo "fn main() {}" > tundra-cli/src/main.rs

RUN cargo fetch

COPY . .

RUN cargo build --release -p executor

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=build /app/target/release/executor /usr/local/bin/executor

EXPOSE 8080

CMD ["executor"]