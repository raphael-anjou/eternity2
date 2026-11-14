#!/usr/bin/env python3
"""A4 byte-identity check: every pre-existing field of every pre-existing row,
timing fields excepted, has the same JSON text in the new file.

    python3 -I scripts/identity.py <old.jsonl> <new.jsonl>
"""
import json
import sys

TIMING = {"ns_per_eval", "stat_seconds", "smc_seconds", "ctx_seconds", "seconds"}


def raw_fields(line):
    """Map key -> raw JSON text of its value, using the decoder's spans."""
    d = json.JSONDecoder()
    s = line.strip()
    out, i = {}, 1
    while s[i] != "}":
        k, j = d.raw_decode(s, i)
        j = s.index(":", j) + 1
        _, e = d.raw_decode(s, j)
        out[k] = s[j:e]
        i = e
        if s[i] == ",":
            i += 1
    return out


def key(f):
    return (f.get("board"), f.get("dir"), f.get("depth"), f.get("done"), "profile_log10_width" in f)


old = [raw_fields(l) for l in open(sys.argv[1]) if l.strip()]
new = {key(raw_fields(l)): raw_fields(l) for l in open(sys.argv[2]) if l.strip()}
bad = checked = 0
for f in old:
    g = new.get(key(f))
    if g is None:
        print("missing row", key(f))
        bad += 1
        continue
    for k, v in f.items():
        if k in TIMING:
            continue
        checked += 1
        if g.get(k) != v:
            bad += 1
            print("differs", key(f), k)
print(f"rows {len(old)}, fields compared {checked}, differences {bad}: {'IDENTICAL' if bad == 0 else 'NOT IDENTICAL'}")
sys.exit(1 if bad else 0)
