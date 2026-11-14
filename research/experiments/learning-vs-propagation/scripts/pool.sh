#!/bin/sh
# learning-vs-propagation: detached job pool. Starts the next job of QUEUE whenever fewer than MAX job.sh processes
# of this directory run (counting jobs started by any earlier queue). One thread per job.
# usage: nohup sh pool.sh QUEUE MAX > /dev/null 2>&1 < /dev/null &
cd "$(dirname "$0")"
while read a b; do
  while [ "$(pgrep -f 'sh job.sh' | wc -l)" -ge "$2" ]; do sleep 20; done
  sh job.sh $a $b &
  sleep 2
done < "$1"
wait
echo "POOL $1 FINISHED $(date +%H:%M:%S)" >> done.txt
