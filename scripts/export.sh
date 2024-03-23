#!/usr/bin/env bash

# Ensure we are in the project directory
scriptdir="$(dirname "$0")"
cd "$scriptdir" && cd ../ || exit;

scripts/build.sh \
&& cd frontend \
&& npm install \
&& npm run build \
&& cd ../ \
&& mkdir -p build/site \
&& cp -r frontend/build/* build/site/ \
&& cp -r scripts/run.sh working_dir/secrets working_dir/migrations build/;

