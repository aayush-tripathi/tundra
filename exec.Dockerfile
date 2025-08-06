FROM rust:1.79 AS build        
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY tundra/Cargo.toml tundra/Cargo.toml
COPY executor/Cargo.toml executor/Cargo.toml
RUN cargo fetch --workspace    

COPY . .
RUN cargo build --release -p executor


FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/executor /usr/local/bin/executor
EXPOSE 8080
CMD ["executor"]
