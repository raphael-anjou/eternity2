#!/bin/zsh
# Second analysis batch: each step is its own run, capped at 14 minutes, one core, nice 10.
cd "$(dirname "$0")"
export PYTHONUNBUFFERED=1
R="nice -n 10 uv run --with numpy --with scipy --with python-flint python"
timeout 840 ${=R} r6_integer_and_power.py clue 22 > r6_clue.out 2>&1
timeout 840 ${=R} r6_integer_and_power.py coarse 3 > r6_coarse.out 2>&1
timeout 840 ${=R} r6_integer_and_power.py cell 128 144 > r6_cell.out 2>&1
timeout 840 ${=R} r6_integer_and_power.py ip 192 45 8 > r6_ip.out 2>&1
timeout 840 ${=R} r6_integer_and_power.py modp > r6_modp.out 2>&1
timeout 840 ${=R} r7_nonabelian_quotients.py 8 > r7_nonabelian_quotients.out 2>&1
echo BATCH_DONE
