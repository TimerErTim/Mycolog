#!/usr/bin/env bash

# Ensure we are in the project directory
scriptdir="$(dirname "$0")"
(cd "$scriptdir" && cd ../) || exit;

cargo +nightly -Z unstable-options -C backend/ build --bin mycolog --release --features prod-env \
&& mkdir -p build \
&& cp backend/target/release/mycolog build/;
