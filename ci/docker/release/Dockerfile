ARG CONTAINER_REGISTRY
ARG NAGGER_BUILD_TAG

FROM rust:1.98.1
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
        quilt libclang-dev && \
    apt-get clean && \
    cargo install --version 6.1.0 cargo-dylint dylint-link --locked && \
    # components should cover at least components in existing_lints/rust-toolchain
    rustup toolchain install nightly-2026-06-25 --profile minimal \
        --component llvm-tools,rustc-dev

ENV NAGGER_PATH=/dist
