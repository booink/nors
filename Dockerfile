FROM rust:1.47-slim-buster

RUN apt-get update -y \
    && apt-get install -y libssl-dev pkg-config

RUN cargo install cargo-watch cargo-make

WORKDIR /app
ADD . .

RUN cargo build --bin nors
