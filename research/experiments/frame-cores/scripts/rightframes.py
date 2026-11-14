"""Write the frames of a puzzle's planted solution (or of every brute-force
solution) as frames-file lines.
  python3 -I rightframes.py planted PUZ PREFIX      (true frame from the planted board)
  python3 -I rightframes.py brute BRUTE.json PREFIX (every solution frame)
"""
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402

mode, src, prefix = sys.argv[1:4]
if mode == "planted":
    puz = common.load_puz(src)
    fr = [t for t in puz["planted"] if common.is_border(puz["n"], t[0])]
    frames = [fr]
else:
    frames = json.load(open(src))["frames"]
for i, fr in enumerate(frames):
    print(prefix + str(i) + " " + " ".join("%d %d %d" % tuple(t) for t in fr))
