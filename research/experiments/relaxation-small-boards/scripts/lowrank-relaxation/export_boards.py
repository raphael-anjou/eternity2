"""TOY BOARDS ONLY. Writes tile lists for iso.c (copied from ../geometry-isolation) from the
shared painter: 8x8 frame 4 with inner 8 to 11, seeds 0-5, to pick the 8x8 board by its solution
count. Usage: python3 -I export_boards.py"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from common import gen
os.makedirs(os.path.join(HERE, "boards"), exist_ok=True)
for ci in (8, 9, 10, 11):
    for s in range(6):
        t = gen(8, 4, ci, s)
        if t is None:
            print("none", ci, s); continue
        with open(os.path.join(HERE, "boards", f"b_8_4_{ci}_{s}.txt"), "w") as f:
            f.write(f"8 {len(t)}\n" + "".join(" ".join(map(str, u)) + "\n" for u in t))
