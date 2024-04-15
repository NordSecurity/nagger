FROM rust:1.77.0

RUN set -eux; \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    cargo install --version 2.4.3 cargo-dylint dylint-link && \
    rustup update nightly-2024-01-25
