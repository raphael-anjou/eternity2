#!/bin/sh
# usage: sh launch.sh JOBFILE PARALLEL   (detached; one thread per job)
cd "$(dirname "$0")"
nohup sh -c "xargs -P $2 -L 1 sh job.sh < $1; echo QUEUE $1 FINISHED \$(date +%H:%M:%S) >> done.txt" > /dev/null 2>&1 < /dev/null &
