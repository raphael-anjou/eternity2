"""Do the capped beta = 0.25 runs of results.txt end in a cycle too?
Run: uv run --quiet --with numpy --with scipy python -I stall025.py
6x6, 4 ring + 7 other colours, seed 1, layouts only, starts 0 and 1 (both capped at 60,000 in results.txt)."""
import sys, os
sys.argv = ["extra.py", "none"]
src = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "extra.py")).read().split('if __name__ == "__main__":')[0]
exec(src)
pieces = inst(6, 4, 7, 1)
for run in (0, 1):
    m = dm.DM(6, pieces, 12, sets="A", form="a", seed=run)
    it, lay, lays, norms, gaps = trace(m, 60000, 0.25, 6000)
    period = next((p for p in range(1, 3000) if all(lays[i] == lays[i - p] for i in range(len(lays) - 3000, len(lays)))), None)
    print(f"run {run}: stop={it}, distinct layouts in the last 6,000 iterations: {len(set(lays))}, exact period: {period}, "
          f"mismatches of the last layout: {m.mismatches(lay)}, max|x| at 54,001 and 60,000: {norms[0]:.1f}, {norms[-1]:.1f}")
