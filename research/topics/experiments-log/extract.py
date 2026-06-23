#!/usr/bin/env python3
"""Extract the experiments log from the research vault's concept notes.

Each concept page carries YAML frontmatter with a `status:` (built / partial /
refuted / unbuilt / wont-do) and a one-line `description:`. This rolls all of them
up into the site's data file, so the public log is a faithful, regenerable view of
every attempt and its outcome.

Usage (from this directory, with the vault checked out alongside the site repo):
    python3 extract.py > ../../../web/src/data/experiments.json

The vault path is resolved relative to the site repo root; adjust VAULT if your
layout differs.
"""
import os, re, json, sys
from collections import Counter

HERE = os.path.dirname(os.path.abspath(__file__))
VAULT = os.path.normpath(os.path.join(HERE, "../../../../v2/vault/concepts"))

def bucket(s):
    s = s.lower()
    if "refut" in s: return "refuted"
    if "wont" in s or "won't" in s: return "wont-do"
    if "partial" in s: return "partial"
    if "unbuilt" in s: return "idea"
    if any(k in s for k in ("built","measured","fixed","documented","research","open")): return "built"
    return "other"

def main():
    out = []
    for fn in sorted(os.listdir(VAULT)):
        if not fn.endswith(".md") or fn.startswith("_"):
            continue
        txt = open(os.path.join(VAULT, fn), encoding="utf-8", errors="replace").read()
        name = fn[:-3]
        m = re.search(r"^---\n(.*?)\n---", txt, re.S)
        fm = m.group(1) if m else ""
        ms = re.search(r"^status:\s*(.+)$", fm, re.M)
        raw = ms.group(1).strip().strip('"').strip("`") if ms else "unknown"
        md = re.search(r"^description:\s*(.+)$", fm, re.M)
        desc = md.group(1).strip().strip('"') if md else ""
        if not desc:
            body = txt[m.end():] if m else txt
            for line in body.splitlines():
                line = line.strip()
                if line and not line.startswith(("#", "**", "---")):
                    desc = line
                    break
        out.append({
            "id": name,
            "title": name.replace("-", " "),
            "status": bucket(raw),
            "summary": desc.strip()[:280],
        })
    out.sort(key=lambda x: x["id"])
    sys.stderr.write("buckets: %s\n" % dict(Counter(x["status"] for x in out)))
    json.dump({"count": len(out), "experiments": out}, sys.stdout, indent=0)

if __name__ == "__main__":
    main()
