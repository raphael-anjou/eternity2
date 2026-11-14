#!/bin/sh
# Start scripts/host_run.sh detached on a remote HOST (ssh -f -n).
# Usage: scripts/launch.sh HOST STAGE P
h=$1; st=$2; p=$3
ssh -F "$SSH_CONFIG" -o BatchMode=yes -f -n "$h" "cd ~/work/t15/research/experiments/five-clue-quota && nohup sh scripts/host_run.sh $h $st $p > host-$st.log 2>&1 < /dev/null &"
