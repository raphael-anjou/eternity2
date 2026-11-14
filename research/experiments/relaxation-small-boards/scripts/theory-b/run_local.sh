#!/bin/bash
# Experiment 5 queue, 8 at a time (all of m1 after study C ended, coordinator's message),
# started once the four convex runs of pass 3 have ended.
cd "$(dirname "$0")"
while [ ! -f raw/local/DONE ]; do sleep 5; done
mkdir -p raw/loc && date > raw/loc/START
xargs -P 8 -I{} sh -c '{}' < jobs_local.txt
date > raw/loc/DONE
