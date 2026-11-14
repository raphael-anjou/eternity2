"""Bottom arcs of frames (row scan from row 15 places these first): the
bottom row plus k - 1 cells up each side, 2k + n - 2 cells.
  python3 -I arcs.py FRAMES N K [K ...]  -> lines "<id>k<K> pos piece rot ..."
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common  # noqa: E402

frames, n, ks = sys.argv[1], int(sys.argv[2]), [int(x) for x in sys.argv[3:]]
for fid, fr in common.load_frames(frames):
    for k in ks:
        cells = {(n - 1) * n + c for c in range(n)}
        for r in range(n - k, n - 1):
            cells |= {r * n, r * n + n - 1}
        sub = sorted(t for t in fr if t[0] in cells)
        assert len(sub) == 2 * k + n - 2
        print("%sk%d %s" % (fid, k, " ".join("%d %d %d" % t for t in sub)))
