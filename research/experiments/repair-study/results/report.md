# Repair study — grid report

140 runs · 10 corner-pinned variants · 60 s each · seed 1 · single core.

Score = matched edges of 480 (canonical rescore). Lift = final − starting board. `last best` is the mean iteration the global best last improved; compare it to `iters` to see how early the run stalled. Iters/sec is never compared across families.


## start

| variant | delta | mean score | mean lift | best | last best | iters | accept |
|:--|:--|--:|--:|--:|--:|--:|--:|
| START-RANDOM | start from a random board instead of a greedy construction | 325.6 | 307.6 | 331 | 16.2K | 6619K | 0.1 |
| START-RARE | start from a rarest-color-first greedy construction (Selby/Riordan rarity) | 373.3 | 15.6 | 381 | 17.4K | 7781K | 0.1 |

## destroy

| variant | delta | mean score | mean lift | best | last best | iters | accept |
|:--|:--|--:|--:|--:|--:|--:|--:|
| GREEDY-MISMATCH | the plain loop: greedy start, destroy mismatched cells, greedy refill, keep if not worse | 365.5 | 17.3 | 372 | 5.2K | 8249K | 0.2 |
| RANDOM-DESTROY | destroy random cells instead of mismatched ones (geometry-blind control) | 400.5 | 52.3 | 409 | 3405.3K | 6359K | 0.7 |
| BAND-DESTROY | destroy the worst band of two rows instead of scattered mismatched cells | 348.6 | 0.4 | 355 | 0 | 1741K | 0.0 |
| COMPONENT-DESTROY | destroy one connected mismatch component plus a one-cell halo | 350.5 | 2.3 | 358 | 138 | 2073K | 0.0 |

## repair

| variant | delta | mean score | mean lift | best | last best | iters | accept |
|:--|:--|--:|--:|--:|--:|--:|--:|
| REPAIR-JITTER | break greedy-refill score ties with a seeded coin (controlled exploration) | 364.9 | 16.7 | 371 | 2.3K | 7946K | 0.3 |
| REPAIR-SMALL | destroy at most six mismatched cells instead of twelve (small-hole baseline) | 362.9 | 14.7 | 369 | 38.0K | 16912K | 0.7 |
| REPAIR-EXACT | rebuild the small hole exactly by bounded assignment instead of greedily | 359.6 | 11.4 | 369 | 4.5K | 5776K | 1.0 |

## accept

| variant | delta | mean score | mean lift | best | last best | iters | accept |
|:--|:--|--:|--:|--:|--:|--:|--:|
| ACCEPT-STRICT | keep only strict improvements (no sideways moves) | 360.6 | 12.4 | 368 | 210.5K | 7872K | 0.0 |
| ACCEPT-ANNEAL | accept worsening moves under a cooling simulated-annealing temperature | 374.9 | 26.7 | 388 | 200.9K | 8543K | 0.6 |
| ACCEPT-LATE | accept against the score 40 iterations ago (late-acceptance hill climbing) | 369.4 | 21.2 | 376 | 1.8K | 8161K | 0.3 |

## restart

| variant | delta | mean score | mean lift | best | last best | iters | accept |
|:--|:--|--:|--:|--:|--:|--:|--:|
| RESTART-KICK | after 400 stalled iterations, randomly kick 24 cells to escape the basin | 364.9 | 16.7 | 372 | 158.3K | 6167K | 0.2 |
| RESTART-REVERT | on a stall, revert to the best board so far instead of kicking (softer perturbation) | 365.4 | 17.2 | 372 | 23.3K | 8456K | 0.2 |
