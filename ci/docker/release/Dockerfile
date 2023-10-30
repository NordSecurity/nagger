ARG CONTAINER_REGISTRY
ARG NAGGER_BUILD_TAG

FROM rust:1.72
LABEL org.opencontainers.image.authors="info@nordsec.com"
LABEL org.opencontainers.image.source=https://github.com/NordSecurity/nagger

COPY dist /dist
RUN set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt-get update && \
    apt-get -y install \
        # general deps        
        pkg-config libssl-dev build-essential \
        # libdrop deps
        cmake \
        # Libtelio dependencies
        quilt && \
    apt-get clean && \
    cargo install --version 2.4.3 cargo-dylint dylint-link && \
    # This is the last 1.72.0 nightly build that works with rust-clippy.
    rustup update nightly-2023-07-02

ENV NAGGER_PATH=/dist
