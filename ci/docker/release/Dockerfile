ARG CONTAINER_REGISTRY
ARG NAGGER_BUILD_TAG

FROM rust:1.77.0
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
    cargo install --version 3.0.0 cargo-dylint dylint-link && \
    rustup update nightly-2024-01-25

ENV NAGGER_PATH=/dist
