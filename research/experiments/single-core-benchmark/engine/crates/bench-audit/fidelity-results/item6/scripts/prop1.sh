#!/bin/sh
# args: label csv cap priv bwflags seeds...  (csv relative to ~/work/d2 on m1)
H=$(dirname "$0")
LABEL=$1; CSV=$2; CAP=$3; PRIV=$4; BWFLAGS=$5; shift 5
scp -F $SSH_CONFIG -o BatchMode=yes -q "$H/prop1_remote.sh" m1:work/d2/prop1_remote.sh
ssh -F $SSH_CONFIG -o BatchMode=yes m1 "sh ~/work/d2/prop1_remote.sh $LABEL $CSV $CAP $PRIV '$BWFLAGS' $*"
mkdir -p "$H/prop"
scp -F $SSH_CONFIG -o BatchMode=yes -q -r "m1:work/d2/prop/$LABEL" "$H/prop/"
python3 -I "$H/prop1.py" "$H/prop/$LABEL"
