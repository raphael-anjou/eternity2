#!/bin/sh
# TOY. Stops the first x86 launch (its SCS log stayed in the stdout buffer) and restarts the driver.
cd "$(dirname "$0")"
for p in $(ps -u "$(id -un)" -o pid=,args= | awk '/run_board\.py 6 4 5|controls_c\.py 6 4 5|raw\/nearctl /{print $1}'); do kill "$p" 2>/dev/null; done
sleep 3
mkdir -p raw_x86/aborted_1950
mv raw_x86/log_6_4_5_* raw_x86/log_ctrl_* raw_x86/driver.out raw_x86/aborted_1950/ 2>/dev/null
echo "$(date '+%Y-%m-%d %H:%M:%S') restart: first launch (19:50) stopped after about 15 min because the SCS iteration log stayed in the stdout buffer; its logs are in aborted_1950/" >> raw_x86/PROGRESS.txt
setsid nohup sh x86_driver.sh "${1:-mkl}" > raw_x86/driver.out 2>&1 < /dev/null &
