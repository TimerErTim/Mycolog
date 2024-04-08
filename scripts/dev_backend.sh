#!/usr/bin/env bash

# Ensure we are in the project directory
scriptdir="$(dirname "$0")"
cd "$scriptdir" && cd ../ || exit;

cd working_dir/ && cargo +nightly -Z unstable-options run --manifest-path ../backend/Cargo.toml --bin mycolog --features dev-env;
