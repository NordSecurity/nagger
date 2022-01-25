ARG CONTAINER_REGISTRY
ARG NAGGER_BUILD_TAG

FROM ${CONTAINER_REGISTRY}/low-level-hacks/automation/nagger/build:${NAGGER_BUILD_TAG}
LABEL org.opencontainers.image.authors="info@nordsec.com"

COPY dist /dist
RUN set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt-get update; \
    apt-get -y install \
        # libdrop deps
        cmake \
        # Libtelio dependencies
        quilt; \
    apt-get clean

ENV NAGGER_PATH=/dist
