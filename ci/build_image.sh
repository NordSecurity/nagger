#!/usr/bin/env bash
set -euxo pipefail

source $(dirname $0)/env.sh

print_help() {
    echo "Usage: ci/build_image.sh <build|push> <build|release>"
    exit 1
}

case "${2:-}" in
    build)
        DOCKERFILE="$SCRIPT_DIR/docker/build/Dockerfile"
        IMAGE_TAG="${CONTAINER_REGISTRY}/low-level-hacks/automation/nagger/build:${NAGGER_BUILD_TAG}"
        ARGS=""
        ;;
    release)
        DOCKERFILE="$SCRIPT_DIR/docker/release/Dockerfile"
        IMAGE_TAG="${CONTAINER_REGISTRY}/low-level-hacks/automation/nagger/release:${CI_COMMIT_TAG}"
        ARGS="--build-arg CONTAINER_REGISTRY=${CONTAINER_REGISTRY} \
              --build-arg NAGGER_BUILD_TAG=${NAGGER_BUILD_TAG}"
        ;;
    *)
        echo "Unrecognized image '${2:-}'"
        print_help
esac

case "${1:-}" in
    build)
        docker build -f "$DOCKERFILE" -t "$IMAGE_TAG" ${BASE_DIR} ${ARGS}
        ;;
    push)
        if $(docker manifest inspect $IMAGE_TAG &> /dev/null); then
            echo "'$IMAGE_TAG' already exists in the registry, make sure to bump the version!"
            exit 1
        fi
        docker push "$IMAGE_TAG"
        ;;
    *)
        echo "Unrecognized action '${1:-}'"
        print_help
esac