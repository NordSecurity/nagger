FROM rust:1.98.1

RUN set -eux; \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    cargo install --version 6.1.0 cargo-dylint dylint-link --locked && \
    rustup update nightly-2026-06-25
