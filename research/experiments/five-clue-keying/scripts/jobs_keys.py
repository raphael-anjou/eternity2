"""The keying list and random control of PREREG.md (same definitions as jobs.py)."""
import itertools, random

KEYS = [(r, a, b) for r in range(1, 6) for a, b in itertools.combinations(range(6, 23), 2)]
RANDOM = [KEYS[i] for i in random.Random(20261113).sample(range(len(KEYS)), 5)]


def kid(k):
    return 'D' if k == 'D' else '-'.join(map(str, k))
