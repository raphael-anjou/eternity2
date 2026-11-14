"""learning-vs-propagation, second pass. TOY boards only. Stdlib only.
Reuses analyze.py's parsed results R and adds: (1) per arm and size, finished / cut at the time limit / cut by the
machine-b reboot (04:30:03) / killed by the wall timeout with no counts / not run; (2) the paired comparisons restricted to
boards where both arms finished; (3) the same comparisons in solver seconds.
usage: python3 -I analyze_extra.py"""
import contextlib, io, math, os, re, statistics, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
with contextlib.redirect_stdout(io.StringIO()):
    import analyze as A  # noqa: E402
R, med, boot = A.R, A.med, A.boot


def secs(arm, n, s):
    """solver-reported seconds: clasp 'Time' (wall), K and dyn 'sec', rowscan from the done file (wall)"""
    for d in ("../results/raw/machine-a", "../results/raw/machine-b"):
        f = os.path.join(HERE, d, "%s_%d_%d.out" % (arm, n, s))
        if os.path.exists(f):
            t = open(f).read()
            m = re.search(r"^c Time\s*:\s*([\d.]+)s", t, re.M)
            if m:
                return float(m.group(1))
            m = re.findall(r'"sec": ?([\d.]+)', t)
            if m:
                return float(m[-1])  # the last one: K lines also carry "sec" inside "first"
    for d in ("../results/raw/done_machine-a.txt", "../results/raw/done_machine-b.txt"):
        for line in open(os.path.join(HERE, d)):
            if line.startswith("%s_%d_%d " % (arm, n, s)):
                return float(re.search(r"sec=(\d+)", line).group(1))
    return None


def status(arm, n, s):
    r = R.get((arm, n, s))
    if r is None:
        return "notrun"
    if arm.endswith("f") and r.get("sols", 0) >= 1:
        return "fin"  # clasp -n 1 prints "Models : 1+" on success; that is not a cut
    if not r.get("cens"):
        return "fin"
    if not any(k in r for k in ("conflicts", "tried", "nodes")):
        return "wallkill"
    t = secs(arm, n, s)
    return "limit" if t is None or t >= 3400 else "reboot"


ARMS = ["T", "AC", "WD", "U0", "U1", "L0", "L1", "C", "K", "U0f", "U1f", "L0f", "L1f", "Cf"]
print("# (1) per arm and size: finished / cut at the 3500 s limit / cut by the machine-b reboot / killed at 3600 s wall with no counts / not run")
for n in (5, 6, 7):
    for arm in ARMS:
        st = [status(arm, n, s) for s in range(1, 11)]
        c = {k: st.count(k) for k in ("fin", "limit", "reboot", "wallkill", "notrun")}
        cut = [s for s in range(1, 11) if st[s - 1] not in ("fin", "notrun")]
        print("n=%d %-4s fin %2d limit %2d reboot %2d wallkill %2d notrun %2d  cut boards %s" % (n, arm, c["fin"], c["limit"], c["reboot"], c["wallkill"], c["notrun"], cut))

print("\n# 7x7 per board: conflicts (or branches) and seconds; * = cut")
for arm in ("AC", "U0", "U1", "L0", "L1", "C", "K"):
    row = []
    for s in range(1, 11):
        r = R.get((arm, 7, s), {})
        w = r.get(A.WORK[arm])
        row.append("%s%s/%ss" % ("-" if w is None else "%.3g" % w, "*" if r.get("cens") else "", "-" if secs(arm, 7, s) is None else "%.0f" % secs(arm, 7, s)))
    print("%-3s %s" % (arm, " ".join(row)))
print("work per second on cut 7x7 runs (median): " + ", ".join("%s %.0f" % (arm, med([R[(arm, 7, s)][A.WORK[arm]] / secs(arm, 7, s) for s in range(1, 11) if status(arm, 7, s) in ("limit", "reboot")])) for arm in ("U0", "U1", "L0", "L1", "C", "K") if any(status(arm, 7, s) in ("limit", "reboot") for s in range(1, 11))))

PAIRS = [("U1", "U0"), ("L1", "L0"), ("K", "U0"), ("C", "U0"), ("K", "AC"), ("U1", "AC"), ("C", "AC"), ("U0", "AC")]
print("\n# (2) paired ratio(a) - ratio(b) in bits, boards where BOTH arms finished (exhaustive)")
for a, b in PAIRS:
    for n in (5, 6, 7):
        v = [A.ratio(a, n, s)[0] - A.ratio(b, n, s)[0] for s in range(1, 11) if status(a, n, s) == "fin" and status(b, n, s) == "fin"]
        if v:
            lo, hi = boot(v)
            print("%s-%s n=%d boards %2d median %+.2f range %+.2f..%+.2f boot95 %+.2f..%+.2f" % (a, b, n, len(v), med(v), min(v), max(v), lo, hi))
        else:
            print("%s-%s n=%d boards  0" % (a, b, n))

print("\n# (3) seconds: log2(seconds of b / seconds of a), positive = a faster. Boards where both finished; then 7x7 with cut runs as bounds")
for a, b in PAIRS:
    for n in (5, 6, 7):
        v = [math.log2(max(secs(b, n, s), 0.01) / max(secs(a, n, s), 0.01)) for s in range(1, 11) if status(a, n, s) == "fin" and status(b, n, s) == "fin"]
        if v:
            print("%s-%s n=%d both finished: boards %2d median %+.2f range %+.2f..%+.2f" % (a, b, n, len(v), med(v), min(v), max(v)))
    up = [s for s in range(1, 11) if status(a, 7, s) in ("limit", "reboot") and status(b, 7, s) == "fin"]
    dn = [s for s in range(1, 11) if status(b, 7, s) in ("limit", "reboot") and status(a, 7, s) == "fin"]
    if up or dn:
        print("   7x7: %s cut while %s finished on boards %s (%s at least %s s vs %s %s s); %s cut while %s finished on boards %s" % (
            a, b, up, a, [round(secs(a, 7, s)) for s in up], b, [round(secs(b, 7, s)) for s in up], b, a, dn))
print("\n# median seconds per arm and size, finished runs only (8 jobs in parallel on machine-a for n=5,6; U0/U1 at n=7 on machine-b)")
for n in (5, 6, 7):
    print("n=%d " % n + "  ".join("%s %s" % (arm, "%.1f" % med([secs(arm, n, s) for s in range(1, 11) if status(arm, n, s) == "fin"]) if any(status(arm, n, s) == "fin" for s in range(1, 11)) else "-") for arm in ("T", "AC", "WD", "U0", "U1", "L0", "L1", "C", "K")))
