#!/bin/sh
# Rerun of the solve-path-6 commands behind results.txt (referee, 2026-11-12).
# usage: sh rerun.sh > rerun.out 2>&1
D="$(dirname "$0")/../solve-path-6"
R="uv run --quiet --with numpy --with scipy python -I $D/dm.py"
echo "== profile"; python3 -I "$D/profile.py"
echo "== dfs"
$R dfs 4 3 3 3; $R dfs 4 3 4 3; $R dfs 5 3 4 3; $R dfs 5 3 5 3
$R dfs 6 4 6 2; $R dfs 6 4 7 2; $R dfs 6 4 8 2; $R dfs 7 4 11 2
echo "== 5x5 c=5"
$R dm 5 3 5 3 5 20000 A a 0.25
$R dm 5 3 5 3 5 20000 ARK a 0.25
$R dm 5 3 5 1 5 20000 A a 1.0 1
echo "== 6x6"
$R dm 6 4 8 2 3 40000 A a 0.25
$R dm 6 4 7 2 3 60000 A a 0.25
$R dm 6 4 8 2 3 20000 ARK a 0.25
$R dm 6 4 7 2 3 20000 ARK a 0.25
echo "== 7x7"
$R dm 7 4 11 2 3 50000 A a 0.25
$R dm 7 4 11 1 4 20000 ARK a 0.25 1
echo "== 8x8"
$R dm 8 5 12 1 2 25000 ARK a 0.25
