#!/usr/bin/env bash
set -euox

source $(dirname $0)/env.sh

if [[ ! -d "${BASE_DIR}/dist" ]]; then
    mkdir "${BASE_DIR}/dist"
fi

TARGET="x86_64-unknown-linux-gnu"

pushd "${BASE_DIR}/existing_lints"
cargo build --target ${TARGET} --release

cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${UNWRAP_CHECK_LINT_BIN}" "${BASE_DIR}/dist/"
cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${MPSC_BLOCKING_SEND_LINT_BIN}" "${BASE_DIR}/dist/"
cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${LARGE_FUTURES_LINT_BIN}" "${BASE_DIR}/dist/"
cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${INDEX_ACCESS_LINT_BIN}" "${BASE_DIR}/dist/"
