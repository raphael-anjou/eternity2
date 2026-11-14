"""Shared helpers for the review of growth-law.md. Stdlib only (python3 -I).

Independent of growth-law/model.py: own generator copy, own first-moment
formulas, own loaders. The C enumerator (enum.c) is compiled once into the
directory named by RV_BIN_DIR (default: the system temp directory).
"""
import json, math, os, random, subprocess, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "../../../../../.."))
EXP = os.path.join(ROOT, "research", "experiments")
BIN_DIR = os.environ.get("RV_BIN_DIR", tempfile.gettempdir())
ENUM = os.path.join(BIN_DIR, "enum")


def ensure_enum():
    if not os.path.exists(ENUM):
        subprocess.check_call(["cc", "-O2", "-o", ENUM, os.path.join(HERE, "enum.c")])


def run_enum(mode, cap, boards, allrot=False):
    """boards: list of (tag, n, pieces). Returns one dict per board."""
    ensure_enum()
    text = "\n".join("%s %d %s" % (tag, n, " ".join(str(v) for p in pieces for v in p)) for tag, n, pieces in boards) + "\n"
    args = [ENUM, mode, str(int(cap))] + (["allrot"] if allrot else [])
    out = subprocess.run(args, input=text, capture_output=True, text=True, check=True).stdout
    return [json.loads(l) for l in out.splitlines()]


# ------------------------------------------------------------ geometry
def seams(n):
    """(frame seams, interior-colour seams); frame = both cells on the same rim line."""
    F, I = [], []
    for y in range(n):
        for x in range(n):
            if x + 1 < n:
                (F if y in (0, n - 1) else I).append(((x, y), (x + 1, y)))
            if y + 1 < n:
                (F if x in (0, n - 1) else I).append(((x, y), (x, y + 1)))
    return F, I


def rot_key(e):
    return min(tuple(e[(i - r) % 4] for i in range(4)) for r in range(4))


def planted_board(n, c, cf, rng, distinct=True, scramble=True):
    """Copy of the painter: each band's colours cycled over its seams, then shuffled.
    Frame colours 1..cf, interior colours cf+1..cf+c. With distinct=True, boards with
    two pieces equal up to rotation are rejected and redrawn (the Rust generator
    repairs them by in-band seam swaps instead). Returns (pieces, redraws)."""
    F, I = seams(n)
    redraws = 0
    while True:
        col = {}
        for band, base, k in ((F, 0, cf), (I, cf, c)):
            cols = [base + (i % k) + 1 for i in range(len(band))]
            rng.shuffle(cols)
            for s, v in zip(band, cols):
                col[s] = v
        pieces = []
        for y in range(n):
            for x in range(n):
                pieces.append((col.get(((x, y - 1), (x, y)), 0), col.get(((x, y), (x + 1, y)), 0),
                               col.get(((x, y), (x, y + 1)), 0), col.get(((x - 1, y), (x, y)), 0)))
        if not distinct or len(set(rot_key(p) for p in pieces)) == len(pieces):
            break
        redraws += 1
        if redraws > 20000:
            return None, redraws
    if scramble:
        rng.shuffle(pieces)
        out = []
        for p in pieces:
            r = rng.randrange(4)
            out.append(tuple(p[(i - r) % 4] for i in range(4)))
        pieces = out
    return pieces, redraws


def iid_board(n, c, cf, rng):
    """Random (not planted) piece set: every half-edge colour independent and uniform."""
    pieces = []
    for y in range(n):
        for x in range(n):
            rim = (y == 0, x == n - 1, y == n - 1, x == 0)  # U R D L
            k = sum(rim)
            e = []
            for i in range(4):
                if rim[i]:
                    e.append(0)
                elif k and (rim[(i + 1) % 4] or rim[(i + 3) % 4]):
                    e.append(rng.randrange(cf) + 1)        # side running along the rim
                else:
                    e.append(cf + rng.randrange(c) + 1)    # inward side or interior piece
            pieces.append(tuple(e))
    return pieces


def ladder_instances(ladder, n=None, role="test"):
    out = []
    for line in open(os.path.join(EXP, "hard-ladder/instances/%s-%s.jsonl" % (ladder, role))):
        r = json.loads(line)
        if n is None or r["n"] == n:
            out.append((r["id"], r["n"], [tuple(p) for p in r["puzzle"]["pieces"]], r))
    return out


# ------------------------------------------------------------ first moments
def counts(E, k):
    b, x = divmod(E, k)
    return [b + (1 if i < x else 0) for i in range(k)]


def ln_arrangements(n):
    w = n - 2
    P = w * w
    return math.log(24) + math.lgamma(4 * w + 1) + math.lgamma(P + 1) + P * math.log(4)


def ln_dfact_even(e):
    """ln (2e-1)!!, real e allowed."""
    return math.lgamma(2 * e + 1) - e * math.log(2) - math.lgamma(e + 1)


def lnS_naive(n, c, cf=5):
    """Independent uniform colours: 1/cf per frame seam, 1/c per other seam."""
    w = n - 2
    return ln_arrangements(n) - 4 * (n - 1) * math.log(cf) - 2 * w * (w + 1) * math.log(c)


def ln_frame_match_balanced(n, cf=5):
    """Ring seams join a clockwise-facing side to an anticlockwise-facing side, and the
    painter gives each colour f_k of each: P(random such pairing is monochromatic) = prod f_k! / E_F!."""
    EF = 4 * (n - 1)
    return sum(math.lgamma(f + 1) for f in counts(EF, cf)) - math.lgamma(EF + 1)


def ln_int_match_balanced(n, c):
    """Interior band, exact colour counts, half-edges paired freely: prod (2e_k-1)!! / (2E_I-1)!!.
    Real c is handled with e_k = E_I / c for every colour."""
    w = n - 2
    EI = 2 * w * (w + 1)
    if abs(c - round(c)) < 1e-12:
        ek = counts(EI, int(round(c)))
        return sum(ln_dfact_even(e) for e in ek) - ln_dfact_even(EI)
    return c * ln_dfact_even(EI / c) - ln_dfact_even(EI)


def lnS_balanced(n, c, cf=5):
    """First moment for the painter's ensemble (exact colour counts), all four turns counted."""
    return ln_arrangements(n) + ln_frame_match_balanced(n, cf) + ln_int_match_balanced(n, c)


def lnF_naive(n, cf=5):
    return math.log(24) + math.lgamma(4 * (n - 2) + 1) - 4 * (n - 1) * math.log(cf)


def lnF_balanced(n, cf=5):
    return math.log(24) + math.lgamma(4 * (n - 2) + 1) + ln_frame_match_balanced(n, cf)


def solve_c(f, lo=1.2, hi=60.0, target=0.0):
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if f(mid) > target:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


def owen(n):
    P = (n - 2) ** 2
    return math.exp((math.lgamma(P + 1) + P * math.log(4)) / (2 * P))


# ------------------------------------------------------------ statistics
def km_median(vals):
    v = sorted(vals)
    return v[max(1, math.ceil(len(v) / 2)) - 1]


def load_rows(pattern_dirs):
    """Raw ladder rows (jsonl) from a list of globs relative to research/experiments."""
    import glob
    rows = []
    for pat in pattern_dirs:
        for f in sorted(glob.glob(os.path.join(EXP, pat))):
            for line in open(f):
                r = json.loads(line)
                if r.get("error"):
                    continue
                r["_file"] = os.path.relpath(f, EXP)
                rows.append(r)
    return rows
