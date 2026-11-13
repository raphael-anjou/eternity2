"""Single-move boards at 478 around a solution, counted from the bag.

Reads engine/data/official_eternity2.csv (top, right, bottom, left; 65535 = grey).
Stdlib only, single thread:  python3 -I defect2_moves.py

A move is "single" when it changes one cell (a turn in place) or two cells (a
swap). Starting from any solution, a move lands at 478 (two broken joints, the
parity floor) when:

  * swap: two pieces of the same kind that, once each is turned to sit where
    the other sat (grey sides kept on the rim), agree on three of their four
    sides. Each cell then breaks exactly the one joint across the differing
    side. Exception: the two pieces sit side by side across that side.
  * turn: an interior piece turned in place keeps exactly two of its four
    sides. For a half turn that is "exactly one opposite pair equal"; for a
    quarter turn (90 or 270, same count) "exactly two cyclically adjacent
    pairs equal". Each (piece, turn) is a separate board.

This counts moves that work in EVERY solution, so it is a floor on the
number of 478 boards one move away from any solution, not a ceiling: swaps of
adjacent pieces and other moves can add more.
"""
import os
from itertools import combinations

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.join(HERE, "..", "..", "..", "..", "..", "engine", "data", "official_eternity2.csv")


def load():
    rows = [l.strip() for l in open(CSV) if l.strip()][1:]
    return [tuple(0 if int(x, 2) == 65535 else int(x, 2) for x in l.split(",")[:4]) for l in rows]


def turn(p, k):
    return tuple(p[(i - k) % 4] for i in range(4))


def legal_turns(a, b):
    """turns of b that put its grey sides where a has them (all four for interior)"""
    out = []
    for k in range(4):
        q = turn(b, k)
        if all((x == 0) == (y == 0) for x, y in zip(a, q)):
            out.append(q)
    return out


P = load()
assert len(P) == 256
kinds = {"interior": 0, "edge": 1, "corner": 2}
by_kind = {name: [p for p in P if p.count(0) == g] for name, g in kinds.items()}
assert [len(by_kind[k]) for k in kinds] == [196, 56, 4]

pairs = {}
pair_moves = {}
for name, group in by_kind.items():
    n_pairs = 0
    n_moves = 0
    for a, b in combinations(group, 2):
        hits = sum(1 for q in legal_turns(a, b) if sum(x != y for x, y in zip(a, q)) == 1)
        if hits:
            n_pairs += 1
            n_moves += hits
    pairs[name] = n_pairs
    pair_moves[name] = n_moves

inner = by_kind["interior"]
half = [p for p in inner if (p[0] == p[2]) + (p[1] == p[3]) == 1]
quarter = [p for p in inner if sum(p[i] == turn(p, 1)[i] for i in range(4)) == 2]
turn_tiles = {p for p in half} | {p for p in quarter}
turn_moves = len(half) + 2 * len(quarter)

print("near-twin pairs (same kind, grey kept on the rim, three of four sides equal):")
for name in kinds:
    print(f"  {name}: {pairs[name]} pairs, {pair_moves[name]} (pair, alignment) swaps")
total_pairs = sum(pairs.values())
total_swaps = sum(pair_moves.values())
print(f"  total: {total_pairs} pairs, {total_swaps} swaps")
print(f"interior half-turn tiles keeping two sides: {len(half)}")
print(f"interior quarter-turn tiles keeping two sides: {len(quarter)} (each gives two boards, 90 and 270)")
print(f"quarter-turn tiles that are also half-turn tiles: {len(set(half) & set(quarter))}")
print(f"turn moves: {len(half)} + 2 x {len(quarter)} = {turn_moves} on {len(turn_tiles)} tiles")
print(f"single-move 478 boards around any solution, floor from the bag: {total_swaps} + {turn_moves} = {total_swaps + turn_moves}")
print(f"old checker sum (interior pairs + half-turn tiles + quarter-turn tiles): {pairs['interior']} + {len(half)} + {len(quarter)} = {pairs['interior'] + len(half) + len(quarter)}")
