"""Independent rerun: exact count comparison against the committed raw outputs. Writes only under the scratch dir given.
Needs the programs built into scripts/bin/ (sh scripts/build.sh; clasp 3.3.10 copied to scripts/bin/clasp) and uv for CaDiCaL.
usage: python3 -I -B rerun.py <real-prefixes folder> <scratch> ct 0|1 (half of the prefixes each) | sat prefix ...
 ct : T on all 160 prefixes, AC on every prefix whose committed AC search took under 60 s
 sat: U0 and K on a few small prefixes (CNF written to scratch, deleted after)"""
import json, os, re, subprocess, sys, time
F, S, mode = os.path.abspath(sys.argv[1]), sys.argv[2], sys.argv[3]
os.chdir(os.path.join(F, "scripts"))
def raw(p, a):
    for d in ("machine-c", "machine-b", "machine-a"):
        f = os.path.join("..", "results", "raw", d, "%s.%s.out" % (p, a))
        if os.path.exists(f): return open(f).read()
P = sorted(f[:-4] for f in os.listdir("../prefixes") if f.endswith(".dyn"))
ok = bad = 0; t0 = time.time()
if mode == "ct":
    for p in P[int(sys.argv[4])::2]:
        j = json.loads(raw(p, "T").strip().splitlines()[-1])
        r = json.loads(subprocess.run(["nice", "-n", "10", "bin/rowscan_real", "../prefixes/%s.dyn" % p, "250"], capture_output=True, text=True).stdout)
        same = (r["nodes"], r["dead"], r["sols"]) == (j["nodes"], j["dead"], j["sols"]); ok += same; bad += not same
        if not same: print("DIFF T", p, r, j)
        j = json.loads(raw(p, "AC").strip().splitlines()[-1])
        if j["wall"] < 60:
            r = json.loads(subprocess.run(["nice", "-n", "10", "bin/dyn16", "file:../prefixes/%s.dyn" % p, "AC", "ROW", "exact", "100000000000000000", "250"], capture_output=True, text=True).stdout)
            same = all(r[k] == j[k] for k in ("tried", "ctried", "alive", "failed", "sols", "censored")); ok += same; bad += not same
            if not same: print("DIFF AC", p, r, j)
else:
    for p in sys.argv[4:]:
        cnf = os.path.join(S, p + ".cnf")
        subprocess.run([sys.executable, "-I", "-B", "enc_real.py", "../prefixes/%s.dyn" % p, cnf], check=True, capture_output=True)
        t = subprocess.run(["nice", "-n", "10", "bin/clasp", "-q", "--stats=2", "--time-limit=250", "-n", "0", "--no-lookback", "--lookahead=no", "--heuristic=None", cnf], capture_output=True, text=True).stdout
        g = lambda x: int(re.search(r"^c Conflicts\s*:\s*(\d+)", x, re.M).group(1))
        same = g(t) == g(raw(p, "U0")); ok += same; bad += not same; print("U0", p, g(t), g(raw(p, "U0")))
        t = subprocess.run(["nice", "-n", "10", "bin/clasp", "-q", "--stats=2", "--time-limit=250", "-n", "0", cnf], capture_output=True, text=True).stdout
        if "UNSATISFIABLE" in raw(p, "C") and "UNSATISFIABLE" in t:
            same = g(t) == g(raw(p, "C")); ok += same; bad += not same; print("C", p, g(t), g(raw(p, "C")))
        k = subprocess.run(["nice", "-n", "10", "uv", "run", "-q", "--with", "python-sat==1.9.dev22", "python3", "-I", "-B", "cdcl_pysat.py", cnf, "250"], capture_output=True, text=True, cwd=F)
        try:
            r = json.loads(k.stdout.strip().splitlines()[-1]); j = json.loads(raw(p, "K").strip().splitlines()[-1])
            same = (r["conflicts"], r["decisions"], r["sols"], r["clauses"]) == (j["conflicts"], j["decisions"], j["sols"], j["clauses"]); ok += same; bad += not same
            print("K", p, r["conflicts"], j["conflicts"], r["final"], "sec", r["wall"], j["wall"])
        except Exception as e:
            print("K not run", p, k.stderr[-300:])
        os.remove(cnf)
print(mode, "identical", ok, "different", bad, "seconds", round(time.time() - t0))
