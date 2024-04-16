#!/usr/bin/env bash

# Ensure we are in the project directory
scriptdir="$(dirname "$0")"
cd "$scriptdir" && cd ../ || exit;

scripts/build_backend.sh \
&& mkdir -p build/ \
&& cp backend/target/release/mycolog build/ \
&& cp -r scripts/run.sh  working_dir/config working_dir/secrets working_dir/migrations working_dir/schedules working_dir/emails build/;
scripts/build_frontend.sh \
&& mkdir -p build/site/ \
&& cp -r frontend/build/* build/site/;

