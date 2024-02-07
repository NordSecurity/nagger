FROM rust:1.72

RUN set -eux; \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    cargo install --version 2.4.3 cargo-dylint dylint-link && \
# This is the last 1.72.0 nightly build that works with rust-clippy.
    rustup update nightly-2024-02-06
