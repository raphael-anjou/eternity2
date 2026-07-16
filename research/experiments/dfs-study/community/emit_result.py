#!/usr/bin/env python3
"""Rescore one engine's board and print a single results JSONL row.

Dispatches to the right parser by --engine, rescores through the ONE canonical
scorer, and scrapes native throughput / max depth / wall seconds from that
engine's run log. Throughput carries its native unit label and is never
cross-compared between engines. The printed row is one line of JSON.
"""
import argparse
import hashlib
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

from canonical_rescore import score_cells, edges_from_cells  # noqa: E402
import parse_mcgavin  # noqa: E402
import parse_blackwood  # noqa: E402
import parse_bucas_url  # noqa: E402


def scrape_mcgavin(log_text):
    """(nps tiles/s, max_depth, wall_s-ish) from McGavin's status lines."""
    rate = None
    for m in re.finditer(r"Rate:\s*([0-9.]+)", log_text):
        rate = float(m.group(1))
    max_depth = None
    for m in re.finditer(r"Max\s+(\d+)", log_text):
        max_depth = int(m.group(1))
    return {"nps": int(rate) if rate else None, "nps_unit": "tiles/s", "max_depth": max_depth}


def scrape_blackwood(log_text):
    nps = None
    for m in re.finditer(r"nps=(\d+)", log_text):
        nps = int(m.group(1))
    best = None
    for m in re.finditer(r"BEST\s+(\d+)", log_text):
        best = int(m.group(1))
    return {"nps": nps, "nps_unit": "search-nodes/s", "max_depth": best}


def scrape_run_algo(log_text):
    """run_algo (verhaard reimpl) / run_dfs (break-2) RESULT line."""
    d = {}
    m = re.search(r"nps=(\d+)", log_text)
    d["nps"] = int(m.group(1)) if m else None
    m = re.search(r"nps_unit=(\S+)", log_text)
    d["nps_unit"] = m.group(1) if m else "search-nodes/s"
    m = re.search(r"max_depth=(\d+)", log_text)
    d["max_depth"] = int(m.group(1)) if m else None
    return d


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--engine", required=True, choices=["mcgavin", "blackwood", "bucas"])
    ap.add_argument("--board", required=True)
    ap.add_argument("--run-log", required=True)
    ap.add_argument("--grid", required=True)  # unpinned | pinned
    ap.add_argument("--instance", required=True)
    ap.add_argument("--seed", default=None)
    ap.add_argument("--label", required=True)
    ap.add_argument("--display", required=True)
    args = ap.parse_args()

    log_text = ""
    if os.path.exists(args.run_log):
        log_text = open(args.run_log, encoding="latin-1").read()

    if args.engine == "mcgavin":
        cells = parse_mcgavin.parse_file(args.board)
        meta = scrape_mcgavin(log_text)
    elif args.engine == "blackwood":
        cells = parse_blackwood.parse_file(args.board)
        meta = scrape_blackwood(log_text)
    else:  # bucas (verhaard reimpl, break-2)
        # These engines emit a full 1024-char string; a stalled run leaves 'aaaa'
        # (all-rim) cells. No real tile is all-rim, so treat 'aaaa' as empty for an
        # accurate placed count. The score is unaffected either way (rim seams do
        # not count), but placed should reflect the true fill.
        edges_raw = parse_bucas_url.edges_from_url(open(args.board, encoding="latin-1").read())
        mask = [edges_raw[i * 4 : i * 4 + 4] == "aaaa" for i in range(256)]
        cells = parse_bucas_url.cells_from_edges(edges_raw, empty_mask=mask)
        meta = scrape_run_algo(log_text)

    placed = sum(1 for c in cells if c is not None)
    score = score_cells(cells)
    edges = edges_from_cells(cells)
    board_hash = hashlib.sha1(edges.encode()).hexdigest()[:16]

    row = {
        "engine": args.label,
        "display": args.display,
        "grid": args.grid,
        "instance": args.instance,
        "seed": int(args.seed) if args.seed is not None else None,
        "score": score,
        "placed": placed,
        "max_depth": meta.get("max_depth"),
        "nps": meta.get("nps"),
        "nps_unit": meta.get("nps_unit"),
        "board_hash": board_hash,
        "board_edges": edges,
    }
    print(json.dumps(row))


if __name__ == "__main__":
    main()
