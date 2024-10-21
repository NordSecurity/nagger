FROM rust:1.77.0

RUN set -eux; \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    cargo install --version 3.0.0 cargo-dylint dylint-link --locked && \
    rustup update nightly-2024-01-25
