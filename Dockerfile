FROM rust:1.59-slim-bullseye

RUN apt-get update -y \
  && apt-get install -y libssl-dev pkg-config python3 python3-pip locales ruby ruby-dev git build-essential clang cmake libstdc++-10-dev libssl-dev libxxhash-dev zlib1g-dev \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen

WORKDIR /tmp
RUN git clone https://github.com/rui314/mold.git \
  && cd mold \
  && git checkout v1.0.1 \
  && make -j$(nproc) CXX=clang++ \
  && make install

RUN rustup component add clippy
RUN cargo install cargo-watch cargo-make cargo-expand

WORKDIR /app
COPY Cargo.toml Cargo.toml
ADD src src
ADD benches benches
RUN mold -run cargo build --bin nors

COPY bindings/python3/requirements.txt bindings/python3/requirements.txt
RUN cd bindings/python3 && pip3 install --upgrade pip && pip3 install -r requirements.txt

RUN gem install bundler

ADD . .
