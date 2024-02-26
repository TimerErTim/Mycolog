#!/usr/bin/env bash

cargo +nightly -Z unstable-options -C backend/ build --bin mycolog --release --features prod-env \
&& mkdir -p build \
&& cp backend/target/release/mycolog build/;
