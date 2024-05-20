#!/usr/bin/env bash
set -euox

BASE_DIR=$PWD

mkdir -p "${BASE_DIR}/dist"

TARGET="x86_64-unknown-linux-gnu"

pushd "${BASE_DIR}/existing_lints"
cargo build --target ${TARGET} --release

RUST_CHANNEL=nightly-2024-01-25
UNWRAP_CHECK_LINT_BIN=libunwrap_check@${RUST_CHANNEL}-x86_64-unknown-linux-gnu.so
MPSC_BLOCKING_SEND_LINT_BIN=libmpsc_blocking_send@${RUST_CHANNEL}-x86_64-unknown-linux-gnu.so
LARGE_FUTURES_LINT_BIN=liblarge_futures@${RUST_CHANNEL}-x86_64-unknown-linux-gnu.so
INDEX_ACCESS_LINT_BIN=libindex_access_check@${RUST_CHANNEL}-x86_64-unknown-linux-gnu.so
TOKIO_TIME_INTERVAL_BIN=libtokio_time_interval@${RUST_CHANNEL}-x86_64-unknown-linux-gnu.so

cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${UNWRAP_CHECK_LINT_BIN}" "${BASE_DIR}/dist/"
cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${MPSC_BLOCKING_SEND_LINT_BIN}" "${BASE_DIR}/dist/"
cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${LARGE_FUTURES_LINT_BIN}" "${BASE_DIR}/dist/"
cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${INDEX_ACCESS_LINT_BIN}" "${BASE_DIR}/dist/"
cp "${BASE_DIR}/existing_lints/target/${TARGET}/release/${TOKIO_TIME_INTERVAL_BIN}" "${BASE_DIR}/dist/"
