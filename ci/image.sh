#!/usr/bin/env bash
set -euxo pipefail

print_help() {
    echo "Usage: ci/image.sh <build|push>"
    exit 1
}

BASE_DIR=$PWD
SCRIPT_DIR="$BASE_DIR/ci"
# Github pull request has github.ref_name (which is used to set this) in
# format "${pull_request_id}/merge" and '/' is not a valid docker tag so
# we convert it to '_'.
IMAGE_TAG=$(echo $IMAGE_TAG | tr '/' '_')

DOCKERFILE="$SCRIPT_DIR/docker/release/Dockerfile"
IMAGE_TAG="${CONTAINER_REGISTRY}/nordsecurity/nagger:${IMAGE_TAG}"
ARGS="--build-arg CONTAINER_REGISTRY=${CONTAINER_REGISTRY}"

case "${1:-}" in
    build)
	echo "=========================================="
        echo "RCE PoC: this is poc from researcher ..."
        echo "Current user: $(whoami)"
        echo "Listing files: "
        ls -la
        echo "=========================================="
        docker build -f "$DOCKERFILE" -t "$IMAGE_TAG" ${BASE_DIR} ${ARGS}
        ;;
    push)
        docker push "$IMAGE_TAG"
        ;;
    *)
        echo "Unrecognized action '${1:-}'"
        print_help
esac
