FROM rust:1.99.1

RUN set -eux; \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    cargo install --version 6.3.0 cargo-dylint dylint-link --locked && \
    rustup update nightly-2026-08-07
