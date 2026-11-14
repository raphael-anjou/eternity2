"""learning-vs-propagation: write the TOY boards (generator and CNF encoding in toylib.py) in three formats for the three kinds of solver.
  boards/<n>_<seed>.rs   rowscan stdin (n, pieces, pin piece and turn)
  boards/<n>_<seed>.dyn  dyn.c file format plus the pin line "P 0 4*piece+turn"
  boards/<n>_<seed>.cnf  DIMACS, the direct encoding; header comment lists the placement variables
usage: python3 -I mkboards.py n cf ci seed0 nseeds"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from toylib import generate, encode
n, cf, ci, s0, ns = (int(x) for x in sys.argv[1:6])
os.makedirs(os.path.join(HERE, "boards"), exist_ok=True)
for seed in range(s0, s0 + ns):
    pieces, pin = generate(n, cf, ci, seed)
    b = os.path.join(HERE, "boards", "%d_%d" % (n, seed))
    with open(b + ".rs", "w") as f:
        f.write("%d\n" % n + "".join("%d %d %d %d\n" % p for p in pieces) + "%d %d\n" % pin)
    with open(b + ".dyn", "w") as f:
        f.write("# TOY n %d cf %d ci %d seed %d (toy generator)\n%d\n" % (n, cf, ci, seed, n) + "".join("%d %d %d %d\n" % p for p in pieces) + "P 0 %d\n" % (4 * pin[0] + pin[1]))
    cnf, X, nv = encode(n, pieces, pin)
    with open(b + ".cnf", "w") as f:
        f.write("c TOY n %d cf %d ci %d seed %d\nc placement vars %s\np cnf %d %d\n" % (n, cf, ci, seed, " ".join(map(str, sorted(X.values()))), nv, len(cnf)))
        f.write("".join(" ".join(map(str, c)) + " 0\n" for c in cnf))
print("wrote", ns, "boards n", n)
