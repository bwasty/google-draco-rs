FROM rust:1.36.0-stretch

RUN rustup component add rustfmt

RUN apt-get update && apt-get install -y \
    cmake \
    llvm-3.9-dev \
    libclang-3.9-dev \
    clang-3.9

WORKDIR /usr/src/google-draco

# dev optimization
RUN cargo install cargo-build-deps
COPY Cargo.toml Cargo.lock ./
COPY src src
RUN cargo build-deps

COPY . .
ENV REGENERATE_BINDINGS=1
# RUN cargo build
# RUN cargo install bindgen
CMD ["/bin/bash"]
