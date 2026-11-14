#!/bin/sh
# Ship the committed tree to x86 (results excluded) and build there.
set -e
W=.
SSHC="ssh -F $SSH_CONFIG -o BatchMode=yes"
$SSHC x86 'mkdir -p ~/work/d1'
git -C $W archive HEAD research engine | $SSHC x86 'tar -x -C ~/work/d1 --exclude="research/experiments/frame-cores/results/*"'
$SSHC x86 'cd ~/work/d1/research/experiments/frame-cores && sh run.sh build 2>&1 | tail -2'
