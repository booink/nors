FROM quay.io/pypa/manylinux_2_24_x86_64

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
ENV PATH="/root/.cargo/bin:$PATH"

WORKDIR /io/bindings/python3