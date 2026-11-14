#!/bin/sh
# Run a job file with R2 jobs first (the longest), J at a time.
D=$(cd "$(dirname "$0")" && pwd)
{ grep '^R2 ' "$1"; grep -v '^R2 ' "$1"; } | xargs -P "${J:-10}" -L 1 sh "$D/job.sh"
