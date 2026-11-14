#!/bin/sh
# Run a TSV job file with P parallel single-threaded jobs.
# Usage: scripts/run_jobs.sh JOBFILE P     (from anywhere)
cd "$(dirname "$0")/.."
tr '\t' ' ' < "$1" | xargs -P "$2" -L 1 sh scripts/one.sh
