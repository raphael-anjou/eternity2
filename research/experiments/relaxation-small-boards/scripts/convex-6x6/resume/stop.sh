#!/bin/sh
# Stops the resumed 6x6 leak runs (only processes running resume/admm.py 6 4 5 ... leak). The last
# checkpoint (every 100 iterations) is kept; launch.sh resumes from it.
pkill -f 'admm.py 6 4 5 [012] leak 300' && echo stopped
