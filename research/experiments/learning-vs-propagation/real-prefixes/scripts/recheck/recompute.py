"""Independent recompute for the real-prefix part. Own parser, stdlib only. Reads the raw outputs, never the analysis tsv
(except for the final cross-check against it).
usage: python3 -I recompute.py        (any working directory)"""
import json, math, os, random, re, sys
F = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..")
DIRS = ["results/raw/machine-c", "results/raw/machine-b", "results/raw/machine-a"]
ARMS = ["T", "AC", "K", "C", "U0"]
R, where, dup, bad = {}, {}, [], []
ext = {}
for d in DIRS:
    for l in open(os.path.join(F, d, "runs.jsonl")):
        j = json.loads(l)
        if j["arm"] != "ENC":
            if (j["prefix"], j["arm"]) in ext: dup.append((j["prefix"], j["arm"]))
            ext[(j["prefix"], j["arm"])] = j
    for f in os.listdir(os.path.join(F, d)):
        if not f.endswith(".out"): continue
        pid, arm = f[:-4].split(".")
        t = open(os.path.join(F, d, f)).read()
        try:
            if arm in ("C", "U0"):
                w = int(re.search(r"^c Conflicts\s*:\s*(\d+)", t, re.M).group(1))
                m = re.search(r"^c Models\s*:\s*(\d+)(\+?)", t, re.M)
                cut = int("UNSATISFIABLE" not in t)
                sec = float(re.search(r"Solving: ([\d.]+)s", t).group(1))
                ll = re.search(r"Conflict\s*:\s*\d+\s*\(Average Length:\s*([\d.]+)", t)
                r = dict(w=w, sols=int(m.group(1)), cut=cut, sec=sec, ll=float(ll.group(1)) if ll else 0)
            else:
                j = json.loads(t.strip().splitlines()[-1])
                if arm == "T": r = dict(w=j["nodes"], sols=j["sols"], cut=int(j["censored"] != 0), sec=j["wall"])
                elif arm == "AC": r = dict(w=j["tried"], failed=j["failed"], sols=j["sols"], cut=int(j["censored"] != 0), sec=j["wall"])
                else: r = dict(w=j["conflicts"], sols=j["sols"], cut=int(j["censored"] != 0 or j["final"] != 1), sec=j["wall"], dec=j["decisions"])
        except Exception as e:
            bad.append((pid, arm, d, repr(e))); continue
        if (pid, arm) in R: dup.append((pid, arm))
        r["dir"] = d; R[(pid, arm)] = r
P = sorted({p for p, _ in R})
dep = lambda p: int(p[1:].split("_")[0])
print("runs", len(R), "prefixes", len(P), "dup", dup, "bad", bad)
print("per dir prefixes", {d: len({p for (p, a), r in R.items() if r["dir"] == d}) for d in DIRS})
print("prefixes with arms split over dirs:", [p for p in P if len({R[(p, a)]["dir"] for a in ARMS}) != 1])
print("hosts per dir", {d: sorted({ext[k]["host"] for k in ext if R[k]["dir"] == d}) for d in DIRS})
print("nonzero sols:", [(k, r["sols"]) for k, r in R.items() if r["sols"]])
print("killed/rc:", [(k, e["rc"], e["killed"]) for k, e in ext.items() if e["killed"] or e["rc"] not in (0, 10, 20)][:20])
print("start range", min(e["start"] for e in ext.values()), max(e["start"] for e in ext.values()))
moved = open(os.path.join(F, "scripts", "queues", "moved.from_machine-a.txt")).read().split()
moved = [m for m in moved if re.match(r"d\d+_\d+$", m)]
print("moved", len(moved), "dirs", {R[(m, "T")]["dir"] for m in moved}, "earliest start of a moved run", min(ext[(m, a)]["start"] for m in moved for a in ARMS))
cuts = [(p, a, R[(p, a)]["dir"], R[(p, a)]["w"]) for p in P for a in ARMS if R[(p, a)]["cut"]]
print("cut runs", len(cuts)); [print("  ", c) for c in cuts]
L = lambda x: math.log2(x) if x > 0 else float("-inf")
def med(v):
    v = sorted(v); n = len(v); return v[n // 2] if n % 2 else (v[n // 2 - 1] + v[n // 2]) / 2
W = lambda p, a, k="w": R[(p, a)][k] + 1
random.seed(7)
for d in (112, 104, 96, 88):
    Q = [p for p in P if dep(p) == d]
    print("\n== depth", d, "n", len(Q))
    print(" all-zero AC,K,C,U0:", sum(all(R[(p, a)]["w"] == 0 for a in ARMS[1:]) for p in Q), " AC and K zero:", sum(R[(p, "AC")]["w"] == 0 and R[(p, "K")]["w"] == 0 for p in Q))
    T = [R[(p, "T")]["w"] for p in Q]
    print(" T: median log2 %.2f  log2 sum %.2f  log2 mean %.2f  top3 share %.2f" % (L(med(T)), L(sum(T)), L(sum(T) / len(Q)), sum(sorted(T)[-3:]) / max(1, sum(T))))
    S = {a: sum(W(p, a) for p in Q) for a in ARMS}; S["ACf"] = sum(W(p, "AC", "failed") for p in Q)
    print(" sums (count+1):", {a: S[a] for a in S}, " cut in sum:", {a: sum(R[(p, a)]["cut"] for p in Q) for a in ARMS})
    for x, y in (("K", "AC"), ("K", "ACf"), ("K", "U0"), ("C", "AC"), ("C", "U0"), ("U0", "AC"), ("U0", "ACf"), ("AC", "T"), ("K", "T")):
        print("  sum lead %s over %s: %+.2f" % (x, y, L(S[y] / S[x])))
    fin = [p for p in Q if not any(R[(p, a)]["cut"] for a in ARMS)]
    Sf = {a: sum(W(p, a) for p in fin) for a in ARMS}
    print(" finished-on-all-arms subset n=%d: K over AC %+.2f, K over U0 %+.2f, C over AC %+.2f, U0 over AC %+.2f, share of T nodes %.2f" % (len(fin), L(Sf["AC"] / Sf["K"]), L(Sf["U0"] / Sf["K"]), L(Sf["AC"] / Sf["C"]), L(Sf["AC"] / Sf["U0"]), sum(R[(p, "T")]["w"] for p in fin) / max(1, sum(T))))
    kfin = [p for p in Q if not R[(p, "K")]["cut"]]
    print(" K-finished subset n=%d: K over AC %+.2f, K over U0 %+.2f" % (len(kfin), L(sum(W(p, "AC") for p in kfin) / sum(W(p, "K") for p in kfin)), L(sum(W(p, "U0") for p in kfin) / sum(W(p, "K") for p in kfin))))
    lead = [L(W(p, "AC") / W(p, "K")) for p in Q]
    print(" median K over AC tried (cut as counted): %+.2f ; over AC failed %+.2f ; K over U0 %+.2f" % (med(lead), med([L(W(p, "AC", "failed") / W(p, "K")) for p in Q]), med([L(W(p, "U0") / W(p, "K")) for p in Q])))
    nt = [p for p in Q if max(R[(p, "AC")]["w"], R[(p, "K")]["w"]) > 100]
    if nt:
        v = [L(W(p, "AC") / W(p, "K")) for p in nt]
        print(" nontrivial (>100) n=%d: median K over AC %+.2f (cut = upper bound), K over U0 %+.2f, min %+.2f max %+.2f, K fewer than AC on %d, K fewer than U0 on %d" % (len(nt), med(v), med([L(W(p, "U0") / W(p, "K")) for p in nt]), min(v), max(v), sum(x > 0 for x in v), sum(W(p, "U0") > W(p, "K") for p in nt)))
    # bootstrap of the sum lead (upper bound where K cut)
    bs = []
    for _ in range(10000):
        s = [random.choice(Q) for _ in Q]
        bs.append(L(sum(W(p, "AC") for p in s) / sum(W(p, "K") for p in s)))
    bs.sort(); print(" bootstrap 95%% of sum lead K over AC: %+.2f to %+.2f" % (bs[250], bs[9750]))
    globals()["BS%d" % d] = bs; globals()["Q%d" % d] = Q
    # seconds within host
    for h in DIRS:
        Qh = [p for p in Q if R[(p, "T")]["dir"] == h]
        if not Qh: continue
        ss = {a: sum(R[(p, a)]["sec"] for p in Qh) for a in ARMS}
        print("  host %-8s n=%2d secs %s | K vs AC %+.2f  AC vs T %+.2f  U0 vs AC %+.2f | rates AC %.0f/s K %.0f/s U0 %.0f/s T %.1fM/s" % (h, len(Qh), {a: round(ss[a], 1) for a in ARMS}, L(max(ss["AC"], .01) / max(ss["K"], .01)), L(max(ss["T"], .01) / max(ss["AC"], .01)), L(max(ss["AC"], .01) / max(ss["U0"], .01)),
              sum(R[(p, "AC")]["w"] for p in Qh) / max(ss["AC"], .01), sum(R[(p, "K")]["w"] for p in Qh) / max(ss["K"], .01), sum(R[(p, "U0")]["w"] for p in Qh) / max(ss["U0"], .01), sum(R[(p, "T")]["w"] for p in Qh) / max(ss["T"], .01) / 1e6))
    ll = [R[(p, "C")]["ll"] for p in Q if R[(p, "C")]["w"] > 100]
    if ll: print(" clasp default mean learnt clause length, median over nontrivial: %.0f (min %.0f max %.0f)" % (med(ll), min(ll), max(ll)))
random.seed(11)
di = []
for _ in range(10000):
    a = [random.choice(Q104) for _ in Q104]; b = [random.choice(Q88) for _ in Q88]
    di.append(L(sum(W(p, "AC") for p in b) / sum(W(p, "K") for p in b)) - L(sum(W(p, "AC") for p in a) / sum(W(p, "K") for p in a)))
di.sort(); print("\nbootstrap rise 104->88 of sum lead K over AC (upper bound at 88): 95%% %+.2f to %+.2f, max %+.2f, share >= 3: %.4f" % (di[250], di[9750], di[-1], sum(x >= 3 for x in di) / 1e4))
print("\nper-prefix d88 nontrivial, sorted by T:")
for p in sorted(Q88, key=lambda p: -R[(p, "T")]["w"])[:26]:
    r = {a: R[(p, a)] for a in ARMS}
    print("  %s %-8s T %11d AC %8d ACf %8d K %8d%s C %8d%s U0 %8d | K/AC %+.2f K/U0 %+.2f | sec T %.1f AC %.1f K %.1f U0 %.1f" % (p, r["T"]["dir"], r["T"]["w"], r["AC"]["w"], r["AC"]["failed"], r["K"]["w"], "*" if r["K"]["cut"] else " ", r["C"]["w"], "*" if r["C"]["cut"] else " ", r["U0"]["w"], L(W(p, "AC") / W(p, "K")), L(W(p, "U0") / W(p, "K")), r["T"]["sec"], r["AC"]["sec"], r["K"]["sec"], r["U0"]["sec"]))
# compare with memo tsv
import csv
rows = list(csv.DictReader(open(os.path.join(F, "results", "analysis", "per_prefix.out.tsv")), delimiter="\t"))
mm = [(r["prefix"], a) for r in rows for a in ARMS if int(r[a + "_work"]) != R[(r["prefix"], a)]["w"] or int(r[a + "_cut"]) != R[(r["prefix"], a)]["cut"]]
print("\nmismatch with per_prefix.out.tsv:", mm, "rows", len(rows))
