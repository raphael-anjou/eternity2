#!/bin/bash
# Runs jobs_compare.txt, 4 at a time (the coordinator's limit on m1 for this study).
cd "$(dirname "$0")" && mkdir -p raw/cmp && date > raw/cmp/START
xargs -P 4 -I{} sh -c '{}' < jobs_compare.txt
date > raw/cmp/DONE
