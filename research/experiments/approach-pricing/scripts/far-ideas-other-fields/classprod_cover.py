"""Control for probe C: how many random tile classes until the class product in S_6 covers its whole parity coset. Toy, stdlib."""
import random
from itertools import permutations
deg = 6
def ct(p):
    seen=[False]*deg; out=[]
    for i in range(deg):
        if not seen[i]:
            L=0; j=i
            while not seen[j]: seen[j]=True; j=p[j]; L+=1
            out.append(L)
    return tuple(sorted(out))
comp = lambda a,b: tuple(b[a[i]] for i in range(deg))
E = list(permutations(range(deg)))
cls = sorted(set(ct(p) for p in E)); cid = {c:i for i,c in enumerate(cls)}
mem = {}
for p in E: mem.setdefault(cid[ct(p)], []).append(p)
K = len(cls)
prod = [[0]*K for _ in range(K)]
for a in range(K):
    x = mem[a][0]
    for b in range(K):
        m = 0
        for y in mem[b]: m |= 1 << cid[ct(comp(x,y))]
        prod[a][b] = m
sign = [sum(L-1 for L in cls[c]) % 2 for c in range(K)]
invol = [p for p in E if all(p[p[i]]==i for i in range(deg)) and p != tuple(range(deg))]
rng = random.Random(7)
print("tiles  mean_classes_covered  classes_in_coset  share_full")
for k in range(1, 11):
    tot = 0; full = 0; T = 300
    for _ in range(T):
        g = {c: rng.choice(invol) for c in range(10)}
        S = 1
        for _ in range(k):
            w = tuple(range(deg))
            for _ in range(4): w = comp(w, g[rng.randrange(10)])
            c = cid[ct(w)]
            n = 0; a = 0; m = S
            while m:
                if m & 1: n |= prod[a][c]
                m >>= 1; a += 1
            S = n
        cov = bin(S).count("1")
        par = sign[(S & -S).bit_length() - 1]
        coset = sum(1 for c in range(K) if sign[c] == par)
        tot += cov; full += cov == coset
    print(k, round(tot / T, 2), coset, round(full / T, 2))
