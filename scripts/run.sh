#!/usr/bin/env bash

mkdir -p log; touch log/total.log

# This assumes your working directory is inside the build folder

until ./mycolog "$@" 2>&1 | tee -a log/total.log | tspin --bucket-size 0; do
    sleep 5
done
