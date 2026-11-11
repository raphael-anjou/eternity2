#!/usr/bin/env python3
"""Corpus diversity and scoring-convention statistics for e2-strong-boards.zip.

Usage: python3 research/datasets/build/corpus_diversity.py
Writes research/datasets/corpus/diversity.json. Needs numpy and scipy.
Deterministic (the random null uses a fixed seed). Runs in about a minute.
"""
import io, json, os, random, zipfile
import numpy as np
from scipy import sparse

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ZIP = os.path.join(ROOT, "e2-strong-boards.zip")
INST = os.path.join(ROOT, "instances", "e2-16x16-v00.json")
OUT = os.path.join(ROOT, "corpus", "diversity.json")
N = 16

clues = [(h["pos"], h["piece"]) for h in json.load(open(INST))["hints"][:5]]

boards, scores = [], []
with zipfile.ZipFile(ZIP) as z:
    for name in sorted(z.namelist()):
        if "/boards/" in name and name.endswith(".json"):
            b = json.loads(z.read(name))
            boards.append(b["board"])
            scores.append(b["score"])
codes = np.array(boards, dtype=np.int32)          # piece*4 + rot
pieces = codes // 4
scores = np.array(scores)
n = len(scores)

def band(lo, hi):
    return (scores >= lo) & (scores <= hi)

all5 = np.all([pieces[:, p] == q for p, q in clues], axis=0)
bands = [(400, 450), (451, 455), (456, 459), (460, 463), (464, 465), (466, 470)]
clue_by_band = {f"{lo}-{hi}": {"boards": int(band(lo, hi).sum()),
                               "all_five_clues": int((all5 & band(lo, hi)).sum())}
                for lo, hi in bands}

# Nearest-neighbour placement distance (cells whose piece*4+rot differs).
nn_place = np.full(n, 256, dtype=np.int32)
for i in range(0, n, 256):
    blk = codes[i:i + 256]
    d = (blk[:, None, :] != codes[None, :, :]).sum(axis=2)
    for k in range(len(blk)):
        d[k, i + k] = 999
    nn_place[i:i + len(blk)] = d.min(axis=1)

# Shared adjacencies: unordered pairs of pieces that touch, insensitive to
# rotating or shifting a block of the board.
def adj_ids(pc):
    ids = []
    for r in range(N):
        for c in range(N):
            a = pc[r * N + c]
            for rr, cc in ((r, c + 1), (r + 1, c)):
                if rr < N and cc < N:
                    b = pc[rr * N + cc]
                    lo, hi = (a, b) if a < b else (b, a)
                    ids.append(lo * 256 + hi)
    return ids

rows, cols = [], []
for i in range(n):
    ids = adj_ids(pieces[i])
    rows += [i] * len(ids)
    cols += ids
X = sparse.csr_matrix((np.ones(len(rows), dtype=np.int32), (rows, cols)), shape=(n, 65536))
S = (X @ X.T).tolil()
S.setdiag(0)
S = S.tocsr()
nn_adj = np.asarray(S.max(axis=1).todense()).ravel()

rng = random.Random(1)
null_shared = []
for _ in range(2000):
    a = list(range(256)); b = list(range(256))
    rng.shuffle(a); rng.shuffle(b)
    null_shared.append(len(set(adj_ids(a)) & set(adj_ids(b))))

def q(x, p):
    return float(np.percentile(x, p))

out = {
    "boards": n,
    "score_share_451_455": round(float(band(451, 455).mean()), 4),
    "all_five_clues_share": round(float(all5.mean()), 4),
    "all_five_clues_by_score_band": clue_by_band,
    "best_all_five_clues_score": int(scores[all5].max()),
    "nn_placement_distance": {"median": q(nn_place, 50), "p10": q(nn_place, 10),
                              "share_le_10": round(float((nn_place <= 10).mean()), 4),
                              "share_le_25": round(float((nn_place <= 25).mean()), 4),
                              "share_le_60": round(float((nn_place <= 60).mean()), 4),
                              "share_le_100": round(float((nn_place <= 100).mean()), 4)},
    "nn_shared_adjacencies_of_480": {"median": q(nn_adj, 50), "p90": q(nn_adj, 90),
                                     "share_ge_240": round(float((nn_adj >= 240).mean()), 4)},
    "null_random_pair_shared_adjacencies": {"mean": round(sum(null_shared) / len(null_shared), 2),
                                            "max_of_2000": max(null_shared)},
}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
json.dump(out, open(OUT, "w"), indent=2)
print(json.dumps(out, indent=2))
