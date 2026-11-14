"""TOY. re-check: starts and boards the configuration was NOT selected on.
 - starts 10 to 49 at r = 16 for the five memo boards (same init_U, rng 1000 r + start, as export.py);
 - ten new 5x5 boards, painter seeds 5 to 14 (frame 3, inner 4), starts 0 to 9, enumerated.
Writes into a scratch COPY of the crate (argument 1: the copy's root, holding py/ and boards/); the
memo's crate is not touched.
Usage: uv run --with numpy --with scipy python -I 06_fresh_export.py <copy root>"""
import os, sys, importlib.util
root = sys.argv[1]
sys.path.insert(0, os.path.join(root, "py"))
spec = importlib.util.spec_from_file_location("exp", os.path.join(root, "py", "export.py")); E = importlib.util.module_from_spec(spec); spec.loader.exec_module(E)
import numpy as np
from board import Board
from bm import init_U
for seed in range(5):
    B = Board(5, 3, 4, seed)
    for s in range(10, 50):
        init_U(B, 16, np.random.default_rng(1000 * 16 + s)).astype("<f8").tofile(os.path.join(root, "boards", f"b5_s{seed}", "init", f"r16_s{s}.bin"))
    print("starts 10-49 written for seed", seed, flush=True)
E.RANKS = (16,)
for seed in range(5, 15):
    E.export(f"b5_s{seed}", Board(5, 3, 4, seed), equiv=False)
