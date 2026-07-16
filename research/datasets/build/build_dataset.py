#!/usr/bin/env python3
"""Build the public Eternity II dataset from in-repo instances + the local
strong-board corpus.

Two parts:
  A. Benchmark instances (loose JSON): the official puzzle, the 10 corner-pinned
     variants, and the 4 clue sub-puzzles. These are the inputs a solver runs on.
  B. Strong-board corpus (zipped): the distinct boards scoring 400-480, each as
     its verified score + edge string + derived structure. The training data the
     learning algorithms mine.

Everything published is either already public in this repo or is board *content*
(edge strings) with a freshly verified score. All provenance from the private
working store (file names, seeds, internal preset names, source paths, puzzle
labels) is stripped: only board content, the recomputed score, and structure
derived from the board itself survive.

Scores are ALWAYS recomputed from the edge string, never copied from a stored
field, so one stale record in the source cannot poison the release.

Run from the repo root:  python3 research/datasets/build/build_dataset.py
"""

import csv
import glob
import hashlib
import json
import os
import zipfile
from collections import Counter, defaultdict

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
CORPUS_SRC = os.path.join(REPO, "..", "v2", "database-400-480")
VARIANTS_SRC = os.path.join(REPO, "research", "experiments", "dfs-study", "variants")
CLUE_SRC = os.path.join(REPO, "research", "topics", "clue-puzzle-pieces")

OUT = os.path.join(REPO, "research", "datasets")
OUT_INSTANCES = os.path.join(OUT, "instances")
OUT_CORPUS = os.path.join(OUT, "corpus")

W = H = 16


# --------------------------------------------------------------------------
# canonical scorer: matched interior edges from a URDL edge string
# 4 lowercase letters per cell, row-major, order up/right/down/left, 'a' = rim.
# --------------------------------------------------------------------------
def score_from_edges(edges: str):
    if len(edges) < W * H * 4:
        return None

    def cell(i):
        return edges[i * 4 : i * 4 + 4]

    m = 0
    for r in range(H):
        for c in range(W):
            i = r * W + c
            if c + 1 < W and cell(i)[1] == cell(i + 1)[3]:  # right vs left
                m += 1
            if r + 1 < H and cell(i)[2] == cell(i + W)[0]:  # down vs up
                m += 1
    return m


def corner_family(vec):
    """The four corner piece ids (pos 0, 15, 240, 255) — the basin key."""
    return tuple(vec[p] for p in (0, 15, 240, 255))


# --------------------------------------------------------------------------
# Part B: the strong-board corpus
# --------------------------------------------------------------------------
def build_corpus():
    files = sorted(glob.glob(os.path.join(CORPUS_SRC, "*.json")))
    rows = []
    stale = 0
    for f in files:
        try:
            d = json.load(open(f))
        except Exception:
            continue
        raw = d.get("bucas_url", "").split("board_edges=")[-1]
        # the edge value ends at the next URL parameter; take exactly the 256*4
        # URDL letters and drop any &board_types=/&board_pieces= tail.
        raw = raw.split("&")[0]
        edges = "".join(ch for ch in raw if ch.islower())[: W * H * 4]
        if len(edges) != W * H * 4:
            continue
        s = score_from_edges(edges)
        if s is None:
            continue
        if d.get("matched") is not None and d.get("matched") != s:
            stale += 1
        pl = d.get("placement")
        vec = None
        if pl and len(pl) == 256:
            vec = [None] * 256
            good = True
            for i, e in enumerate(pl):
                pos = e.get("pos", i)
                if pos is None or pos >= 256:
                    good = False
                    break
                vec[pos] = e["piece_id"]
            if not good:
                vec = None
        rows.append({"score": s, "edges": edges, "vec": tuple(vec) if vec else None})

    # dedup on edge string (the board content the scorer sees). Exact copies go.
    seen = {}
    for r in rows:
        seen.setdefault(r["edges"], r)
    uniq = list(seen.values())

    # assign basin (corner family) ids from piece vectors where available,
    # falling back to the corner *edge* signature when a board had no placement.
    fam_of = {}
    fam_counter = Counter()
    for r in uniq:
        if r["vec"]:
            fam = corner_family(r["vec"])
        else:
            # corner cells' outward-facing edges as a stable structural key
            e = r["edges"]
            fam = (e[0:4], e[15 * 4 : 15 * 4 + 4], e[240 * 4 : 240 * 4 + 4], e[255 * 4 : 255 * 4 + 4])
        key = hashlib.sha1(repr(fam).encode()).hexdigest()[:8]
        r["family_key"] = key
        fam_counter[key] += 1

    # stable, neutral ids: sort by (score desc, edges) so the id is reproducible
    uniq.sort(key=lambda r: (-r["score"], r["edges"]))
    # renumber families by size (largest = f001) for readable ids
    fam_rank = {k: i + 1 for i, (k, _) in enumerate(fam_counter.most_common())}

    records = []
    for idx, r in enumerate(uniq, 1):
        records.append(
            {
                "id": f"e2b-{idx:05d}",
                "score": r["score"],
                "family": f"f{fam_rank[r['family_key']]:03d}",
                "edges": r["edges"],
            }
        )

    os.makedirs(OUT_CORPUS, exist_ok=True)
    # JSONL
    with open(os.path.join(OUT_CORPUS, "boards.jsonl"), "w") as fh:
        for rec in records:
            fh.write(json.dumps(rec, separators=(",", ":")) + "\n")
    # CSV
    with open(os.path.join(OUT_CORPUS, "boards.csv"), "w", newline="") as fh:
        w = csv.writer(fh)
        w.writerow(["id", "score", "family", "edges"])
        for rec in records:
            w.writerow([rec["id"], rec["score"], rec["family"], rec["edges"]])

    stats = {
        "boards": len(records),
        "exact_copies_removed": len(rows) - len(uniq),
        "stale_scores_corrected": stale,
        "distinct_families": len(fam_counter),
        "score_min": min(r["score"] for r in records),
        "score_max": max(r["score"] for r in records),
        "score_histogram": dict(sorted(Counter(r["score"] for r in records).items())),
        "family_sizes_top10": [n for _, n in fam_counter.most_common(10)],
    }
    return records, stats


def zip_corpus():
    zpath = os.path.join(OUT, "e2-strong-boards.zip")
    with zipfile.ZipFile(zpath, "w", zipfile.ZIP_DEFLATED) as z:
        for name in ("boards.csv", "boards.jsonl"):
            z.write(os.path.join(OUT_CORPUS, name), arcname=f"e2-strong-boards/{name}")
        # README + license travel inside the zip too
        for name in ("README.md", "LICENSE"):
            p = os.path.join(OUT, name)
            if os.path.exists(p):
                z.write(p, arcname=f"e2-strong-boards/{name}")
    return zpath, os.path.getsize(zpath)


# --------------------------------------------------------------------------
# Part A: benchmark instances
# --------------------------------------------------------------------------
def build_instances():
    os.makedirs(OUT_INSTANCES, exist_ok=True)
    manifest = []

    # the 10 corner-pinned variants + official are already public, canonical JSON.
    for f in sorted(glob.glob(os.path.join(VARIANTS_SRC, "variant_*.json"))):
        d = json.load(open(f))
        vid = os.path.basename(f).replace("variant_", "").replace(".json", "")
        rec = {
            "id": f"e2-16x16-v{vid}",
            "family": "official-16x16-pin3corners",
            "width": d["width"],
            "height": d["height"],
            "numColors": d["numColors"],
            "pieces": d["pieces"],
            "hints": d.get("hints", []),
            # interior matched-edge maximum = horizontal + vertical interior edges
            "maxScore": d["width"] * (d["height"] - 1) + d["height"] * (d["width"] - 1),
        }
        out = os.path.join(OUT_INSTANCES, f"{rec['id']}.json")
        json.dump(rec, open(out, "w"), indent=2)
        manifest.append({"id": rec["id"], "family": rec["family"], "size": f"{d['width']}x{d['height']}", "pieces": len(d["pieces"]), "hints": len(rec["hints"]), "maxScore": rec["maxScore"]})

    # the 4 clue sub-puzzles (piece lists already in-repo). Normalize to the same schema.
    clue_files = {
        "clue1": ("clue1_shorter36.txt", 6, 6),
        "clue2": ("clue2_shorter72.txt", 12, 6),
        "clue3": ("clue3_jwortmann.txt", 6, 6),
        "clue4": ("clue4_jwortmann.txt", 12, 6),
    }
    for cid, (fname, cw, ch) in clue_files.items():
        path = os.path.join(CLUE_SRC, "data", fname)
        if not os.path.exists(path):
            continue
        pieces = []
        for line in open(path):
            parts = line.split()
            if not parts:
                continue
            nums = [int(x) for x in parts]
            # tolerate a leading index column: keep the last 4 as URDL colours
            pieces.append(nums[-4:])
        ncol = max((max(p) for p in pieces), default=0) + 1
        rec = {
            "id": f"e2-clue-{cid}",
            "family": "clue-subpuzzle",
            "width": cw,
            "height": ch,
            "numColors": ncol,
            "pieces": pieces,
            "hints": [],
            "maxScore": cw * (ch - 1) + ch * (cw - 1),
        }
        json.dump(rec, open(os.path.join(OUT_INSTANCES, f"{rec['id']}.json"), "w"), indent=2)
        manifest.append({"id": rec["id"], "family": rec["family"], "size": f"{cw}x{ch}", "pieces": len(pieces), "hints": 0, "maxScore": rec["maxScore"]})

    json.dump({"instances": manifest}, open(os.path.join(OUT_INSTANCES, "index.json"), "w"), indent=2)
    return manifest


if __name__ == "__main__":
    print("Part A: benchmark instances")
    inst = build_instances()
    print(f"  wrote {len(inst)} instances to {OUT_INSTANCES}")

    print("Part B: strong-board corpus")
    records, stats = build_corpus()
    print(f"  wrote {stats['boards']} boards (removed {stats['exact_copies_removed']} exact copies, corrected {stats['stale_scores_corrected']} stale score(s))")
    print(f"  {stats['distinct_families']} distinct families, scores {stats['score_min']}..{stats['score_max']}")

    json.dump(stats, open(os.path.join(OUT_CORPUS, "stats.json"), "w"), indent=2)

    zpath, zsize = zip_corpus()
    print(f"  zipped -> {zpath} ({zsize/1024:.0f} KB)")

    # top-level dataset manifest
    manifest = {
        "name": "eternity2-dataset",
        "version": "1.0.0",
        "license": "CC0-1.0",
        "parts": {
            "instances": {"count": len(inst), "path": "instances/"},
            "corpus": {"count": stats["boards"], "zip": "e2-strong-boards.zip", "unpacked": "corpus/"},
        },
        "corpus_stats": stats,
    }
    json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w"), indent=2)
    print("done.")
