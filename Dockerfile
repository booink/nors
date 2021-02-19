FROM rust:1.50-slim-buster

RUN apt-get update -y \
  && apt-get install -y libssl-dev pkg-config python3 python3-pip locales \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen

RUN cargo install cargo-watch cargo-make cargo-expand

WORKDIR /app
ADD . .

RUN cargo build --bin nors
RUN cd bindings/python3 && pip3 install --upgrade pip && pip3 install -r requirements.txt
