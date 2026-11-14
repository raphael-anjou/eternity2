"""Diagnostic: on M10 boards 2 and 5 every single pin leaves 2 solutions.
Pin all cells except one row at a time and report the solution count, to
locate where the second solution differs from the planted one."""
import json, os, subprocess
EXP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BIN = os.path.join(EXP, "engine/target/release/pinmap")
out = open(os.path.join(EXP, "results/diag.jsonl"), "a")
for seed in (2, 5):
    b = os.path.join(EXP, f"boards/n10-f5-c18-s{seed}.txt")
    jobs = ""
    for r in range(10):
        pins = [q for q in range(100) if q // 10 != r]
        jobs += f"diag:{seed}:free-row{r}\t{','.join(map(str, pins))}\n"
    for c in range(10):
        pins = [q for q in range(100) if q % 10 != c]
        jobs += f"diag:{seed}:free-col{c}\t{','.join(map(str, pins))}\n"
    res = subprocess.run([BIN, "rs", "--board", b, "--order", "fwd"], input=jobs, capture_output=True, text=True, check=True).stdout
    for l in res.splitlines():
        out.write(l + "\n")
        d = json.loads(l)
        print(d["label"], d["solutions"])
