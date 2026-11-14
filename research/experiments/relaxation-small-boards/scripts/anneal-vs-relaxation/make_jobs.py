"""TOY BOARDS ONLY. Writes jobs.txt for T1 (independent re-check a separate re-check memo (not included), section 4).
One line per process: tag board runs steps seed. Schedule fixed by the re-check: T 0.8 -> 0.2, snaps 1.
Longest jobs first. Seeds are new (the re-check used 404, 555, 666)."""
jobs = []
for b in (0, 1):
    for r in range(5):
        jobs.append(("a3e9", f"7_4_7_{b}", 1, 3_000_000_000, 4000 + 10 * b + r))
for b in (1, 0):
    for c in range(10):
        jobs.append(("a2e8", f"7_4_7_{b}", 10, 200_000_000, 3000 + 10 * b + c))
for s in range(5):
    for c in range(4):
        jobs.append(("a2e8", f"6_4_5_{s}", 5, 200_000_000, 2000 + 10 * s + c))
for s in range(5):
    jobs.append(("a2e6", f"6_4_5_{s}", 20, 2_000_000, 1000 + s))
with open("jobs.txt", "w") as f:
    for j in jobs:
        f.write(" ".join(map(str, j)) + "\n")
with open("jobs_calib.txt", "w") as f:
    for c in range(2):
        f.write(f"calib2e8 6_4_5_0 10 200000000 {5000 + c}\n")
with open("jobs_probe.txt", "w") as f:
    for c in range(8):
        f.write(f"probe2e7 7_4_7_0 1 20000000 {99900 + c}\n")
