"""The five clue pieces: which colours they carry, against the 196 interior
pieces. B18 of how-the-puzzle-was-made found their numbers high (p = 0.027);
under the sort a high number means "smallest colour label is high". Sharper
readings of the same fact, all post hoc, with Monte Carlo tails over uniform
5-subsets of the 196 interior pieces.
python3 -I 11_clue_piece_colours.py
"""
import random, sys, os, collections
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "gap-5"))
os.environ.setdefault("E2_CSV", "engine/data/official_eternity2.csv")
import e2common as E
pieces, hints = E.load()
inner = [i for i, p in enumerate(pieces) if 0 not in p]
print("hints (number, x, y, rot):", hints)
cl = [h[0] - 1 for h in hints]
for i in cl: print("  piece %3d colours %s  min label %d" % (i + 1, pieces[i], min(pieces[i])))
def stats(idx):
    sides = [c for i in idx for c in pieces[i]]
    return (sum(i + 1 for i in idx),                       # B18
            sum(1 for c in sides if c <= 10),              # sides in the five 24-joint colours (labels 6..10)
            sum(min(pieces[i]) for i in idx),              # sum of smallest labels
            len(set(sides)),                               # distinct colours on the five pieces
            sum(sides))                                    # sum of labels
names = ["sum of numbers", "sides with label 6..10", "sum of min labels", "distinct colours", "sum of labels"]
obs = stats(cl)
rng = random.Random(11); T = 200000
ge = [0] * 5; le = [0] * 5
for _ in range(T):
    s = stats(rng.sample(inner, 5))
    for k in range(5):
        ge[k] += s[k] >= obs[k]; le[k] += s[k] <= obs[k]
for k in range(5):
    print("%-24s observed %5d   P(>=) %.4f   P(<=) %.4f" % (names[k], obs[k], ge[k] / T, le[k] / T))
cnt = collections.Counter(c for i in cl for c in pieces[i])
print("colour counts on the five clue pieces:", sorted(cnt.items()))
