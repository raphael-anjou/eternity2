"""Scaling table for the memo (TOY boards; everything past 5x5 is an estimate).

Method priced: the facially reduced two-set splitting of admm.py, one dense symmetric
eigendecomposition of order D = P + 1 - (2 n^2 - 1) per iteration, double precision.

Inputs, each marked in the memo:
  sizes               counts_and_tree.py (theory memo formulas)
  seconds per eig     one core: measured here at orders 479, 1291, 2917 (raw/eig_time_*.json, CPU
                      seconds, LAPACK dsyevd through Accelerate), extrapolated as D^3 from the
                      largest measured order
                      14 cores: one-core time / 6 (GUESS: dense eigensolvers scale poorly)
                      GPU: 80.4 s for order 50,000, eigenvalues only, cuSOLVER on an H100
                      (arXiv 2410.02170), times 1.5 for the vectors (GUESS), scaled as D^3, with
                      a floor of 20 ms per call (GUESS); needs 5 dense double matrices in 80 GB
  iterations          T(P) = T5 (P / 478)^a with T5 = 4,500 (measured: 3,500 to 6,000 on three 5x5 boards,
                      iterations until the mass off solutions is below 1e-5), a = 1 (optimistic) and a = 3
                      (pessimistic). The one measured step, 4x4 to 5x5, gives a between 1.6 and
                      3.1 (this splitting: 500 to 3,500..4,500; SCS: 175 to 3,025..8,200).
  row scan            first-moment model of counts_and_tree.py plus 1.5 bits (the model is 0.5 to
                      2.1 bits low on three measured sizes and 1.6 to 6.6 low at 8x8 (4,11)), at 1.6e7 nodes per second per core
                      (run log 10:32), linear in cores.

Usage: python3 -I scaling.py
"""
import glob, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
rows = {(d["n"], d["cf"], d["ci"]): d for d in json.load(open(os.path.join(HERE, "raw", "counts_and_tree.json")))}
eig = {}
for f in glob.glob(os.path.join(HERE, "raw", "eig_time_*.json")):
    d = json.loads(open(f).read().strip().splitlines()[-1])
    eig[d["P"] + 1] = d["numpy_eigh_syevd"]["cpu_s"]
Dmax = max(eig)
T5, P5 = 4500, 478


def t_eig_core(D):
    if D <= Dmax:
        ks = sorted(eig)
        lo = max([k for k in ks if k <= D], default=ks[0]); hi = min([k for k in ks if k >= D], default=ks[-1])
        base = lo if abs(lo - D) <= abs(hi - D) else hi
        return eig[base] * (D / base) ** 3
    return eig[Dmax] * (D / Dmax) ** 3


def t_eig_gpu(D):
    return max(0.02, 1.5 * 80.4 * (D / 50000) ** 3)


def fmt(s):
    for unit, k in (("s", 1), ("min", 60), ("h", 3600), ("d", 86400)):
        if s < 100 * k:
            return f"{s / k:.2g} {unit}"
    y = s / (365.25 * 86400)
    return f"{y:.2g} y" if y < 1000 else f"{y:.0e} y".replace("e+0", "e").replace("e+", "e")


out = []
print("eig seconds measured (one core):", eig)
for key in [(5, 3, 4), (6, 4, 5), (7, 4, 7), (8, 5, 8), (10, 5, 10), (12, 5, 12), (16, 5, 17)]:
    d = rows[key]; n = key[0]
    D = d["face_order"]
    mem_gb = 5 * 8 * (d["P"] + 1) ** 2 / 2**30
    its = {a: T5 * (d["P"] / P5) ** a for a in (1, 3)}
    te1, teg = t_eig_core(D), t_eig_gpu(D)
    tree = d["log2_nodes"] + 1.5 if n != 16 else 134.4
    rs1 = 2 ** tree / 1.6e7
    rec = dict(board=f"{n}x{n} ({key[1]},{key[2]})", P=d["P"], zeros=d["zeros"], free=d["free"], equalities=d["equalities_2n2"], D=D,
               mem_5_matrices_GB=round(mem_gb, 2), s_per_iter_1core=te1, s_per_iter_gpu=teg,
               its_opt=its[1], its_pess=its[3],
               core1_opt=fmt(its[1] * te1), core1_pess=fmt(its[3] * te1),
               core14_opt=fmt(its[1] * te1 / 6), core14_pess=fmt(its[3] * te1 / 6),
               gpu_opt=fmt(its[1] * teg) if mem_gb <= 80 else "does not fit", gpu_pess=fmt(its[3] * teg) if mem_gb <= 80 else "does not fit",
               log2_tree=tree, rowscan_1core=fmt(rs1), rowscan_14core=fmt(rs1 / 14))
    out.append(rec)
    print(json.dumps(rec))
json.dump(out, open(os.path.join(HERE, "raw", "scaling.json"), "w"), indent=1)
