"""TOY BOARDS ONLY. One summary table over admm.py PROGRESS files.
Usage: python3 -I collect_table.py PROGRESS_*.txt ...

Classes (the kill line is fixed in ../convex-6x6.md and is not changed here):
  converged = the run's own final line says so (primal, step and dual residuals all below 1e-7).
  converged and exact      mass off solutions per cell below 1e-5 (the memo's exact reading)
  converged and leaking    mass off solutions per cell at or above 1e-5 (the number is given;
                           above 0.05 on a leak run is the kill-line level)
  not converged            no converged final line yet; residuals given. Proves nothing either way.
Only "leak" runs certify exactness; an "l2" point with no leak is a point, not a certificate.
"""
import json, os, sys, time

rows = []
for f in sys.argv[1:]:
    recs = []
    for line in open(f):
        try:
            recs.append(json.loads(line))
        except ValueError:
            pass
    if not recs:
        continue
    start = next((r for r in recs if r.get("start")), {})
    its = [r for r in recs if "it" in r and not r.get("start")]
    fin = next((r for r in reversed(recs) if r.get("final")), None)
    last = its[-1] if its else {}
    host = os.path.basename(os.path.dirname(os.path.abspath(f)))
    obj = start.get("objective", "?"); board = start.get("board")
    mass = (fin or last).get("mass_off_per_cell")
    if fin and fin["status"] == "converged":
        if mass < 1e-5:
            cls = "converged and exact" if obj == "leak" else "converged, no leak at this point (not a certificate)"
        elif mass > 0.05:
            cls = "converged and LEAKING %.3g (above 0.05)" % mass
        else:
            cls = "converged and leaking %.3g" % mass
    elif last:
        cls = "not converged (%s): rp %.1e step %.1e dual %.1e" % ("stopped" if fin else "running", last["r_primal"], last["r_step"], last["r_dual"])
    else:
        cls = "not converged (no iteration logged yet)"
    rows.append((board, obj, host, last.get("it"), last.get("s_per_it"), last.get("t"), last.get("rho"), mass, last.get("viol_max"), last.get("clock"), cls))

rows.sort(key=lambda r: (str(r[0]), r[1]))
print("| board (n,cf,ci,seed) | objective | where | iterations | s/it | elapsed s | rho | mass off per cell | max violation | last line at | class |")
print("|---|---|---|---|---|---|---|---|---|---|---|")
fmt = lambda v, f: ("" if v is None else f % v)
for b, o, h, it, spi, t, rho, m, vm, ck, cls in rows:
    print(f"| {b} | {o} | {h} | {fmt(it, '%d')} | {fmt(spi, '%.3f')} | {fmt(t, '%.0f')} | {fmt(rho, '%.3g')} | {fmt(m, '%.2e')} | {fmt(vm, '%.1e')} | {ck or ''} | {cls} |")

leak6 = [r for r in rows if r[1] == "leak" and r[0] and r[0][0] == 6]
conv = [r for r in leak6 if r[10].startswith("converged")]
over = [r for r in conv if r[7] is not None and r[7] > 0.05]
print(f"\nKill line (6x6 leak runs converged to 1e-7 with mass off solutions above 0.05 per cell on 2 of 3 boards): "
      f"{len(conv)} of {len(leak6)} leak runs converged, {len(over)} above 0.05 -> "
      + ("KILL: not exact at 6x6, stop all engineering on this line" if len(over) >= 2 else
         "passes (lift exact on the converged boards)" if len(conv) == 3 else "no verdict yet"))
print(time.strftime("collected %F %T"))
