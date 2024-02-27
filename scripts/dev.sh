#!/usr/bin/env bash

# Ensure we are in the project directory
scriptdir="$(dirname "$0")"
cd "$scriptdir" && cd ../ || exit;

cd working_dir/ && cargo +nightly -Z unstable-options -C ../backend/ run --bin mycolog --features dev-env;
