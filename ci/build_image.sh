#!/usr/bin/env bash
set -euxo pipefail

print_help() {
    echo "Usage: ci/build_image.sh <build|push> <build|release>"
    exit 1
}

SCRIPT_DIR="$PWD/ci"
BASE_DIR=$PWD

DOCKERFILE="$SCRIPT_DIR/docker/release/Dockerfile"
IMAGE_TAG="${CONTAINER_REGISTRY}/nordsecurity/nagger:${CI_COMMIT_TAG}"
ARGS="--build-arg CONTAINER_REGISTRY=${CONTAINER_REGISTRY}"

case "${1:-}" in
    build)
        docker build -f "$DOCKERFILE" -t "$IMAGE_TAG" ${BASE_DIR} ${ARGS}
        ;;
    push)
        docker push "$IMAGE_TAG"
        ;;
    *)
        echo "Unrecognized action '${1:-}'"
        print_help
esac
