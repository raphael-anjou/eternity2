"""Null distribution of the number of complete frames: boards whose 60 ring
joints are a uniform shuffle of 12 joints of each of 5 frame colours (the
exchangeable painter of Track 10 and of how-the-puzzle-was-made, restricted to
the ring, which is all the frame count depends on). The count of labelled
frames is estimated by the population estimator of Track 10 (enumerate all
children of the kept particles, keep N uniformly without replacement).
Validation: the official ring must give log10 = 37.6077 (exact, gate b).
python3 -I 09_frame_count_null.py <N> <null boards> <seed>
"""
import math, random, sys, statistics as st
sys.path.insert(0, "research/experiments/orders-and-pins/scripts")
import orders as O

def ring_pieces_official():
    corners, edges = [], []
    for p in O.load_official():
        g = [i for i in range(4) if p[i] == 0]
        if len(g) == 2:
            # the two grey sides are adjacent; walking the ring clockwise the piece is entered
            # through the side that follows the greys counter-clockwise and left through the other
            a, b = g
            if (a + 1) % 4 != b: a, b = b, a      # greys at a, a+1
            corners.append((p[(a + 3) % 4], p[(b + 1) % 4]))
        elif len(g) == 1:
            k = g[0]
            edges.append((p[(k + 3) % 4], p[(k + 1) % 4]))
    return corners, edges

def ring_pieces_painted(rng):
    joints = [c for c in range(1, 6) for _ in range(12)]
    rng.shuffle(joints)
    slots = [(joints[i - 1], joints[i]) for i in range(60)]   # slot i: enters by joint i-1, leaves by joint i
    corners = [slots[i] for i in (0, 15, 30, 45)]
    edges = [slots[i] for i in range(60) if i not in (0, 15, 30, 45)]
    return corners, edges

def count(corners, edges, N, rng):
    """log10 of the estimated number of labelled frames."""
    by_in_c = {}; by_in_e = {}
    for i, (a, b) in enumerate(corners): by_in_c.setdefault(a, []).append((1 << i, b))
    for i, (a, b) in enumerate(edges): by_in_e.setdefault(a, []).append((1 << (4 + i), b))
    # slot 0 is a corner: its entering side closes the ring at the end
    parts = [(1 << i, b, a) for i, (a, b) in enumerate(corners)]   # (used, last out colour, colour needed to close)
    logn = math.log10(len(parts))
    for slot in range(1, 60):
        table = by_in_c if slot % 15 == 0 else by_in_e
        kids = []
        last = slot == 59
        for used, c, close in parts:
            for bit, out in table.get(c, ()):
                if not used & bit and (not last or out == close):
                    kids.append((used | bit, out, close))
        if not kids: return float("-inf")
        logn += math.log10(len(kids) / len(parts))
        parts = kids if len(kids) <= N else rng.sample(kids, N)
    return logn

if __name__ == "__main__":
    N = int(sys.argv[1]); B = int(sys.argv[2]); seed = int(sys.argv[3])
    rng = random.Random(seed)
    c, e = ring_pieces_official()
    off = [count(c, e, N, rng) for _ in range(4)]
    offm = math.log10(sum(10 ** x for x in off) / len(off))
    print("official ring: 4 reps", [round(x, 4) for x in off], "log10 mean %.4f (exact 37.6077)" % offm, flush=True)
    vals = []
    for b in range(B):
        c, e = ring_pieces_painted(rng)
        v = count(c, e, N, rng)
        same = sum(1 for a, bb in e if a == bb)
        vals.append(v)
        print("null %3d  log10 frames %.4f  same-colour edge pieces %d" % (b, v, same), flush=True)
    fin = [v for v in vals if v > float("-inf")]
    m, s = st.mean(fin), st.stdev(fin)
    print("null: n %d (zero-frame estimates %d) mean %.4f sd %.4f min %.4f max %.4f" % (len(fin), len(vals) - len(fin), m, s, min(fin), max(fin)))
    print("model (first moment, same census) 37.7136 ; null mean minus model %+.4f" % (m - 37.7136))
    print("official exact 37.6077: z %+.2f ; null boards below it: %d of %d" % ((37.6077 - m) / s, sum(1 for v in fin if v < 37.6077), len(fin)))
