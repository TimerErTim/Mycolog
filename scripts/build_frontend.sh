#!/usr/bin/env bash

# Ensure we are in the project directory
scriptdir="$(dirname "$0")"
cd "$scriptdir" && cd ../ || exit;

cd frontend \
&& npm install \
&& npm run build;
