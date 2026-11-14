### Rung n8-c6 (30 test instances)

First-moment expected solutions 1.11e+07 (ln 16.2), x = c^2/(4(n-2)^2) = 0.250, whole-board critical c = 7.32 (c - c_crit = -1.32).

| entrant | unit | budget | share Q1 / median / Q3 | mean share | median gap | at optimum | no complete board | errors | median wall s | machines |
|---|---|---|---|---|---|---|---|---|---|---|
| break-r1 | placement | 5.12e+07 | 92.86% / 93.75% / 94.64% | 93.99% | 6.25% | 0 | 0 | 0 | 0.2 | m1 |
| break-r2 | placement | 5.12e+07 | 86.61% / 86.61% / 87.50% | 87.20% | 13.39% | 0 | 0 | 0 | 0.0 | m1 |
| sa | move_eval | 2.56e+08 | 95.54% / 96.43% / 96.43% | 96.25% | 3.57% | 0 | 0 | 0 | 100.3 | x86 |
| pt | move_eval | 2.56e+08 | 96.43% / 96.43% / 97.32% | 96.82% | 3.57% | 0 | 0 | 0 | 100.5 | x86 |
| alns | move_eval | 2.56e+08 | 86.83% / 88.39% / 89.29% | 88.15% | 11.61% | 0 | 0 | 0 | 21.8 | m1 |

Work to optimum minus k (Kaplan-Meier median, censored at the budget; reached / 30):

| entrant | k=1 | k=2 | k=3 | k=4 | k=5 | k=6 |
|---|---|---|---|---|---|---|
| break-r1 | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (5) | > budget (13) |
| break-r2 | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (0) |
| sa | > budget (0) | > budget (1) | > budget (3) | 2.56e+08 (20) | 3.2e+07 (30) | 1.6e+07 (30) |
| pt | > budget (0) | > budget (0) | > budget (13) | 4.36e+07 (30) | 6.9e+06 (30) | 3.4e+06 (30) |
| alns | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (0) | > budget (0) |

Criterion (1): entrants in band 2, best below optimum on 30/30, pass = False

| pair | median diff of share | a wins / b wins | Wilcoxon W | p (sign flips) |
|---|---|---|---|---|
| break-r1 vs break-r2 | 6.70 pt | 30 / 0 | 465.0 | 0.00000 |
| break-r1 vs sa | -1.79 pt | 0 / 30 | -465.0 | 0.00000 |
| break-r1 vs pt | -2.68 pt | 0 / 30 | -465.0 | 0.00000 |
| break-r1 vs alns | 5.80 pt | 30 / 0 | 465.0 | 0.00000 |
| break-r2 vs sa | -8.93 pt | 0 / 30 | -465.0 | 0.00000 |
| break-r2 vs pt | -9.82 pt | 0 / 30 | -465.0 | 0.00000 |
| break-r2 vs alns | -0.89 pt | 11 / 18 | -183.0 | 0.04717 |
| sa vs pt | -0.45 pt | 2 / 15 | -129.0 | 0.00090 |
| sa vs alns | 8.48 pt | 30 / 0 | 465.0 | 0.00000 |
| pt vs alns | 8.93 pt | 30 / 0 | 465.0 | 0.00000 |

implementation_level: {"level": "implementation", "verdict": "undetermined", "pairs": []}

family_level: {"level": "family", "pairs": [{"pair": ["break-r1", "sa"], "p": 0.0, "holm_significant": true, "median_diff_expected_positive": -0.017857142857142905, "agrees": false}, {"pair": ["break-r2", "sa"], "p": 0.0, "holm_significant": true, "median_diff_expected_positive": -0.0892857142857143, "agrees": false}, {"pair": ["alns", "sa"], "p": 0.0, "holm_significant": true, "median_diff_expected_positive": -0.08482142857142855, "agrees": false}], "linked_entrants": 4, "criterion2_pass": true, "criterion3_pass": false}

