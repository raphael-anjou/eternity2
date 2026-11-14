node rate r (nodes/s, capped R0-rows runs): {'real': [2399970.562021653, 40], 'null': [2531097.8564474564, 44]}

| set | log10 N_F | log10 N_H | beta_H (bits) |
|---|---|---|---|
| null1101 | 38.008 | 32.634 | 17.85 |
| null1102 | 37.787 | 32.625 | 17.15 |
| null1103 | 37.707 | 32.419 | 17.56 |
| real | 37.603 | 32.412 | 17.24 |
| realfar | 37.603 | 32.412 | 17.24 |

Stage 1: lower median of t_best_plain / t_X, 95% bootstrap interval, censored pairs

| set/stratum | n | R1-rows | R1-spiral | R2 | total-time ratio R1-rows / R1-spiral / R2 | plain censored |
|---|---|---|---|---|---|---|
| null1101/H | 20 | 0.09 [0.03, 1.00] (7) | 0.12 [0.03, 1.00] (5) | 4.92 [1.00, 172.50] (3) | 0.50 / 0.72 / 0.50 | 7 |
| null1101/H/near | 8 | 0.03 [3.12e-03, 0.77] (1) | 0.03 [0.03, 0.78] (1) | 4.92 [0.59, 310.60] (0) | 0.22 / 0.36 / 739.86 | 1 |
| null1101/H/far | 12 | 1.00 [0.03, 1.00] (6) | 1.00 [0.03, 1.19] (4) | 1.00 [0.52, 493.05] (3) | 0.63 / 0.87 / 0.42 | 6 |
| null1101/U | 10 | 1.00 [1.00, 1.50] (0) | 1.00 [0.50, 1.50] (0) | 9.76e-05 [6.24e-05, 1.72e-04] (0) | 227.79 / 94.26 / 0.06 | 0 |
| null1101/U/near | 10 | 1.00 [1.00, 1.50] (0) | 1.00 [0.50, 1.50] (0) | 9.76e-05 [6.24e-05, 1.72e-04] (0) | 227.79 / 94.26 / 0.06 | 0 |
| null1102/H | 20 | 1.00 [0.07, 1.00] (10) | 1.00 [0.06, 1.00] (9) | 1.00 [1.00, 31.99] (8) | 0.65 / 0.75 / 0.26 | 10 |
| null1102/H/near | 6 | 0.02 [6.04e-03, 0.20] (0) | 0.06 [0.02, 0.20] (0) | 31.99 [0.02, 248.08] (0) | 0.09 / 0.14 / 168.17 | 0 |
| null1102/H/far | 14 | 1.00 [0.79, 1.00] (10) | 1.00 [0.07, 1.00] (9) | 1.00 [1.00, 9.51] (8) | 0.84 / 0.88 / 0.25 | 10 |
| null1102/U | 10 | 1.00 [1.00, 1.50] (0) | 1.00 [1.00, 1.00] (0) | 1.23e-04 [8.32e-05, 1.54e-04] (0) | 201.52 / 107.19 / 0.04 | 0 |
| null1102/U/near | 10 | 1.00 [1.00, 1.50] (0) | 1.00 [1.00, 1.00] (0) | 1.23e-04 [8.32e-05, 1.54e-04] (0) | 201.52 / 107.19 / 0.04 | 0 |
| null1103/H | 20 | 0.57 [0.09, 1.00] (6) | 0.26 [0.03, 1.00] (5) | 16.43 [1.00, 486.31] (2) | 0.64 / 0.65 / 0.72 | 6 |
| null1103/H/near | 10 | 0.56 [0.02, 1.00] (3) | 0.26 [9.24e-03, 1.00] (2) | 52.91 [0.02, 2351.88] (0) | 0.73 / 0.71 / 1345.06 | 3 |
| null1103/H/far | 10 | 0.70 [0.03, 1.00] (3) | 0.14 [0.02, 1.00] (3) | 2.27 [0.33, 393.26] (2) | 0.56 / 0.59 / 0.33 | 3 |
| null1103/U | 10 | 1.00 [1.00, 1.50] (0) | 1.00 [0.67, 1.50] (0) | 1.06e-04 [5.87e-05, 1.60e-04] (0) | 240.39 / 99.82 / 0.06 | 0 |
| null1103/U/near | 10 | 1.00 [1.00, 1.50] (0) | 1.00 [0.67, 1.50] (0) | 1.06e-04 [5.87e-05, 1.60e-04] (0) | 240.39 / 99.82 / 0.06 | 0 |
| real/H | 60 | 0.89 [0.04, 1.00] (24) | 0.12 [0.04, 1.00] (22) | 1.00 [1.00, 26.49] (14) | 0.67 / 0.77 / 0.38 | 25 |
| real/H/near | 23 | 0.04 [0.03, 0.19] (5) | 0.05 [0.03, 0.19] (5) | 62.88 [0.61, 559.69] (0) | 0.59 / 0.75 / 1285.65 | 6 |
| real/H/far | 37 | 1.00 [0.06, 1.00] (19) | 1.00 [0.04, 1.00] (17) | 1.00 [1.00, 14.65] (14) | 0.71 / 0.78 / 0.27 | 19 |
| real/U | 30 | 1.50 [1.00, 105.05] (0) | 2.00 [1.33, 39.17] (0) | 2.56e-04 [1.51e-04, 0.12] (0) | 1.48 / 1.45 / 0.23 | 0 |
| real/U/near | 30 | 1.50 [1.00, 105.05] (0) | 2.00 [1.33, 39.17] (0) | 2.56e-04 [1.51e-04, 0.12] (0) | 1.48 / 1.45 / 0.23 | 0 |
| realfar/H | 60 | 1.00 [1.00, 1.00] (34) | 1.00 [1.00, 1.00] (34) | 1.00 [1.00, 4.77] (24) | 0.80 / 0.97 / 0.32 | 37 |
| realfar/H/near | 8 | 0.19 [0.02, 1.00] (2) | 0.19 [0.03, 1.00] (2) | 76.98 [1.39, 1471.78] (0) | 0.53 / 0.68 / 823.73 | 2 |
| realfar/H/far | 52 | 1.00 [1.00, 1.00] (32) | 1.00 [1.00, 1.00] (32) | 1.00 [1.00, 1.00] (24) | 0.84 / 1.01 / 0.29 | 35 |
| null-pooled/H | 60 | 0.70 [0.14, 1.00] (23) | 0.55 [0.08, 1.00] (19) | 6.57 [1.00, 52.91] (13) | 0.59 / 0.71 / 0.38 | 23 |
| null-pooled/U | 30 | 1.00 [1.00, 1.50] (0) | 1.00 [1.00, 1.00] (0) | 1.17e-04 [8.42e-05, 1.30e-04] (0) | 227.47 / 98.76 / 0.06 | 0 |

Stage 2: median v per frame (b_H for H, b_F for U), 95% interval, R2 core size

| set/stratum | n | censored | median v (bits) | 95% CI | secondary: median v_F - beta_H | 95% CI | median R2 core cells | R2 core quartiles | median best-nogood cells | reading | L (secondary, b_F) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| null1101/H | 20 | 4 | 41.92 | [31.39, 119.33] | 24.60 | [14.42, 97.34] | 5.50 | [3, 20] | 5.00 | passes necessary condition | 130.53 |
| null1101/H/near | 8 | 0 | 31.39 | [30.48, 35.77] | 14.22 | [12.71, 20.02] | 3.00 | [3, 4] | 3.00 | passes necessary condition | n/a |
| null1101/H/far | 12 | 4 | 119.33 | [45.59, inf] | 97.34 | [27.12, inf] | 20.00 | [11, inf] | 33.50 | undecided | n/a |
| null1101/U | 10 | 0 | 13.49 | [12.98, 18.60] | 13.49 | [12.98, 18.60] | 2.00 | [2, 2] | 3.00 | passes necessary condition | 21.59 |
| null1101/U/near | 10 | 0 | 13.49 | [12.98, 18.60] | 13.49 | [12.98, 18.60] | 2.00 | [2, 2] | 3.00 | passes necessary condition | n/a |
| null1102/H | 20 | 9 | 91.52 | [32.82, inf] | 78.35 | [15.84, inf] | 20.00 | [3, inf] | 19.00 | undecided | 106.45 |
| null1102/H/near | 6 | 0 | 28.99 | [27.73, 35.19] | 12.39 | [10.83, 20.25] | 3.00 | [3, 3] | 3.00 | passes necessary condition | n/a |
| null1102/H/far | 14 | 9 | inf | [91.04, inf] | inf | [77.17, inf] | inf | [20, inf] | inf | dead | n/a |
| null1102/U | 10 | 0 | 13.49 | [12.61, 14.15] | 13.49 | [12.61, 14.15] | 2.00 | [2, 3] | 3.00 | passes necessary condition | 25.89 |
| null1102/U/near | 10 | 0 | 13.49 | [12.61, 14.15] | 13.49 | [12.61, 14.15] | 2.00 | [2, 3] | 3.00 | passes necessary condition | n/a |
| null1103/H | 20 | 2 | 37.07 | [32.22, 41.92] | 21.23 | [15.95, 25.05] | 4.00 | [3, 10] | 4.00 | passes necessary condition | 69.05 |
| null1103/H/near | 10 | 0 | 32.22 | [28.40, 37.66] | 15.95 | [11.34, 21.34] | 3.50 | [3, 4] | 3.50 | passes necessary condition | n/a |
| null1103/H/far | 10 | 2 | 56.78 | [36.48, inf] | 40.07 | [21.11, inf] | 8.00 | [4, 19] | 8.00 | undecided | n/a |
| null1103/U | 10 | 0 | 13.83 | [10.75, 19.14] | 13.83 | [10.75, 19.14] | 2.00 | [2, 2] | 3.00 | passes necessary condition | 27.95 |
| null1103/U/near | 10 | 0 | 13.83 | [10.75, 19.14] | 13.83 | [10.75, 19.14] | 2.00 | [2, 2] | 3.00 | passes necessary condition | n/a |
| real/H | 60 | 16 | 36.75 | [32.90, 53.19] | 19.61 | [16.54, 37.39] | 5.00 | [3, 21] | 5.00 | passes necessary condition | 71.94 |
| real/H/near | 23 | 0 | 29.83 | [28.30, 33.78] | 14.28 | [12.19, 15.89] | 3 | [3, 4] | 3 | passes necessary condition | n/a |
| real/H/far | 37 | 16 | 75.12 | [42.64, inf] | 66.63 | [25.63, inf] | 16 | [4, inf] | 19 | undecided | n/a |
| real/U | 30 | 0 | 14.43 | [13.50, 24.13] | 14.43 | [13.50, 24.13] | 2.00 | [2, 2] | 3.00 | passes necessary condition | 26.96 |
| real/U/near | 30 | 0 | 14.43 | [13.50, 24.13] | 14.43 | [13.50, 24.13] | 2.00 | [2, 2] | 3.00 | passes necessary condition | n/a |
| realfar/H | 60 | 29 | 86.57 | [46.08, inf] | 63.09 | [28.40, inf] | 17.50 | [5, inf] | 18.00 | undecided | 89.69 |
| realfar/H/near | 8 | 0 | 40.47 | [33.85, 44.03] | 25.07 | [18.27, 26.87] | 5.00 | [4, 5] | 5.00 | passes necessary condition | n/a |
| realfar/H/far | 52 | 29 | inf | [58.43, inf] | 98.23 | [44.16, inf] | 20.50 | [5, inf] | inf | dead | n/a |
| null-pooled/H | 60 | 15 | 39.68 | [34.99, 70.51] | nan | [nan, nan] | 5.50 | [3, 22] | 5.00 | passes necessary condition | n/a |
| null-pooled/U | 30 | 0 | 13.59 | [13.04, 14.07] | nan | [nan, nan] | 2.00 | [2, 2] | 3.00 | passes necessary condition | n/a |

A2 floors (log2 mean of 2^v lower bounds)

| set/stratum | F(0.4) | F(0.25) | F(0.1) | log2 mean | best | reading |
|---|---|---|---|---|---|---|
| null1101/H | 34.45 | 43.94 | 121.74 | 130.53 | 130.53 | no reading (A2 has no pass line) |
| null1101/H/near | 30.45 | 33.77 | 35.59 | 35.86 | 35.86 | no reading (A2 has no pass line) |
| null1101/H/far | 44.68 | 111.60 | 122.47 | 131.27 | 131.27 | no reading (A2 has no pass line) |
| null1101/U | 12.50 | 12.97 | 20.78 | 21.59 | 21.59 | no reading (A2 has no pass line) |
| null1101/U/near | 12.50 | 12.97 | 20.78 | 21.59 | 21.59 | no reading (A2 has no pass line) |
| null1102/H | 29.18 | 32.84 | 68.95 | 106.45 | 106.45 | no reading (A2 has no pass line) |
| null1102/H/near | 28.02 | 29.23 | 36.99 | 36.99 | 36.99 | no reading (A2 has no pass line) |
| null1102/H/far | 29.28 | 33.03 | 69.46 | 106.96 | 106.96 | no reading (A2 has no pass line) |
| null1102/U | 12.57 | 12.41 | 25.89 | 25.89 | 25.89 | no reading (A2 has no pass line) |
| null1102/U/near | 12.57 | 12.41 | 25.89 | 25.89 | 25.89 | no reading (A2 has no pass line) |
| null1103/H | 36.34 | 38.52 | 67.19 | 69.05 | 69.05 | no reading (A2 has no pass line) |
| null1103/H/near | 34.03 | 35.93 | 36.46 | 37.09 | 37.09 | no reading (A2 has no pass line) |
| null1103/H/far | 41.99 | 68.50 | 69.63 | 70.05 | 70.05 | no reading (A2 has no pass line) |
| null1103/U | 12.83 | 12.70 | 27.94 | 27.95 | 27.95 | no reading (A2 has no pass line) |
| null1103/U/near | 12.83 | 12.70 | 27.94 | 27.95 | 27.95 | no reading (A2 has no pass line) |
| real/H | 31.24 | 34.48 | 59.07 | 71.94 | 71.94 | no reading (A2 has no pass line) |
| real/H/near | 29.20 | 32.75 | 36.93 | 45.75 | 45.75 | no reading (A2 has no pass line) |
| real/H/far | 31.30 | 35.92 | 71.45 | 72.63 | 72.63 | no reading (A2 has no pass line) |
| real/U | 22.72 | 22.45 | 24.41 | 26.96 | 26.96 | no reading (A2 has no pass line) |
| real/U/near | 22.72 | 22.45 | 24.41 | 26.96 | 26.96 | no reading (A2 has no pass line) |
| realfar/H | 33.13 | 38.64 | 51.19 | 89.69 | 89.69 | no reading (A2 has no pass line) |
| realfar/H/near | 39.64 | 42.03 | 41.32 | 42.49 | 42.49 | no reading (A2 has no pass line) |
| realfar/H/far | 31.54 | 36.20 | 51.40 | 89.90 | 89.90 | no reading (A2 has no pass line) |

Bottom arcs (A1 item 4), real H frames

| arc | cells | refuter | frames | unsat | within 60 s | within 65 s | sat | capped | median s (unsat) | median b_H | median b_F |
|---|---|---|---|---|---|---|---|---|---|---|---|
| k15 | 44 | R0-rowsup-partial | 60 | 22 | 21 | 21 | 0 | 38 | 0.02 | 100.80 | 113.43 |
| k15 | 44 | R2-glucose | 60 | 42 | 42 | 42 | 0 | 18 | 0.04 | 100.80 | 113.43 |
| k14 | 42 | R2-glucose | 60 | 41 | 41 | 41 | 0 | 19 | 0.05 | 100.69 | 111.91 |
| k13 | 40 | R2-glucose | 60 | 34 | 34 | 34 | 0 | 26 | 0.07 | 98.73 | 109.44 |
| k12 | 38 | R2-glucose | 60 | 30 | 30 | 30 | 0 | 30 | 0.05 | 96.33 | 105.52 |

arc readings (A1): {'R0-rowsup-partial': 'k = 13 not run for this refuter', 'R2-glucose': 'below the claimed break-even'}

| arc | cells | exponent to parent | mean c (s) | log2 mean c (nodes) | budget (log2 nodes) | budget (s) | can kill at cap | below break-even |
|---|---|---|---|---|---|---|---|---|
| k15 | 44 | -1.4 | 89.49 | 27.7 | 25.5 | 19.77 | True | False |
| k14 | 42 | 3.5 | 1155.42 | 31.4 | 29.0 | 223.70 | True | False |
| k13 | 40 | 3.9 | 1.88e+04 | 35.4 | 32.9 | 3339.50 | True | False |
| k12 | 38 | 4.1 | 3.22e+05 | 39.5 | 37.0 | 5.73e+04 | False | False |

uniform real frames: {'frames': 30, 'refuted': 30, 'best_nogood_within_2_of_a_corner': 28, 'median_best_plain_nodes': 3.5, 'beta_H_bits': 17.24313216213183}

Cores (the minimising nogood per frame)

| set/stratum | frames | refuted | source R2 / R0-rows / R0-spiral | contiguous | contains a corner cell | median ring distance to a corner | median b (bits) | median log2 C | R2 refuted | R2 median conflicts |
|---|---|---|---|---|---|---|---|---|---|---|
| null1101/H | 20 | 16 | 13 / 0 / 3 | 2 | 0 | 1 | 20.28 | 18.31 | 17 | 16 |
| null1101/U | 10 | 10 | 2 / 2 / 6 | 1 | 0 | 1 | 12.39 | 1.00 | 10 | 2 |
| null1102/H | 20 | 11 | 8 / 1 / 2 | 0 | 0 | 1 | 15.80 | 15.76 | 12 | 14 |
| null1102/U | 10 | 10 | 1 / 4 / 5 | 1 | 0 | 1 | 12.15 | 1.58 | 10 | 1 |
| null1103/H | 20 | 18 | 16 / 1 / 1 | 2 | 0 | 1 | 17.50 | 18.56 | 18 | 19 |
| null1103/U | 10 | 10 | 2 / 5 / 3 | 1 | 0 | 1 | 12.41 | 1.58 | 10 | 2 |
| real/H | 60 | 44 | 33 / 9 / 2 | 5 | 0 | 1 | 16.13 | 16.81 | 46 | 15 |
| real/U | 30 | 30 | 13 / 10 / 7 | 1 | 0 | 1 | 12.05 | 2.00 | 30 | 2 |
| realfar/H | 60 | 31 | 26 / 3 / 2 | 1 | 0 | 1 | 20.99 | 17.22 | 36 | 17 |

verdict: {'stage1_kill': 'dead', 'stage2_reading': 'passes necessary condition', 'arcs_A1': {'R0-rowsup-partial': 'k = 13 not run for this refuter', 'R2-glucose': 'below the claimed break-even'}, 'arcs_A2': {'k15': False, 'k14': False, 'k13': False, 'k12': False}, 'stage2_A2': 'no reading (A2 has no pass line)'}
