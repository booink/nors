FROM rust:1.50-slim-buster

RUN apt-get update -y \
  && apt-get install -y libssl-dev pkg-config python3 python3-pip locales ruby ruby-dev \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen

RUN rustup component add clippy
RUN cargo install cargo-watch cargo-make cargo-expand cargo-clippy

WORKDIR /app
COPY Cargo.toml Cargo.toml
ADD src src
ADD benches benches
RUN cargo build --bin nors

COPY bindings/python3/requirements.txt bindings/python3/requirements.txt
RUN cd bindings/python3 && pip3 install --upgrade pip && pip3 install -r requirements.txt

RUN gem install bundler

ADD . .

CMD ["cargo", "watch", "-x", "test", "-x", "clippy"]
