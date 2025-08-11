FROM rust:1.89.0

RUN set -eux; \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    cargo install --version 4.1.0 cargo-dylint dylint-link --locked && \
    rustup update nightly-2025-06-12
