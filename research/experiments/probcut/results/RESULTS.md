| rung | solver | solved (Wilson 95%) | KM q25 / median / q75 nodes | median wall ms | total wall s | first-pass solved | passes used | false-cut rate (pass 1) |
|---|---|---|---|---|---|---|---|---|
| 6x6 c=5 | scb-gacolor_ac3_random | 30/30 (0.89-1.00) | 620 / 951 / 1,940 | 6.2 | 0.3 |  |  |  |
| 6x6 c=5 | hp-dfs | 30/30 (0.89-1.00) | 27,904 / 41,823 / 62,556 | 23.7 | 0.8 |  |  |  |
| 6x6 c=5 | fc-dfs | 30/30 (0.89-1.00) | 25,961 / 38,866 / 56,883 | 4.2 | 0.2 |  |  |  |
| 6x6 c=5 | fc-luby | 30/30 (0.89-1.00) | 32,290 / 74,830 / 144,530 | 7.6 | 0.3 |  |  |  |
| 6x6 c=5 | probcut-v1 | 30/30 (0.89-1.00) | 26,858 / 171,321 / 248,763 | 14.7 | 0.5 | 27/30 | 1:27, 2:3 | 0.70 |
| 6x6 c=5 | probcut-v2 | 30/30 (0.89-1.00) | 26,946 / 41,315 / 77,397 | 4.8 | 0.2 | 30/30 | 1:30 | 0.07 |
| 7x7 c=6 | scb-gacolor_ac3_random | 16/30 (0.36-0.70) | 272,281 / 921,773 / > cap | 7347.2 | 173.1 |  |  |  |
| 7x7 c=6 | hp-dfs | 0/30 (0.00-0.11) | > cap / > cap / > cap | 652.5 | 19.9 |  |  |  |
| 7x7 c=6 | fc-dfs | 0/30 (0.00-0.11) | > cap / > cap / > cap | 99.8 | 3.2 |  |  |  |
| 7x7 c=6 | fc-luby | 0/30 (0.00-0.11) | > cap / > cap / > cap | 101.7 | 3.3 |  |  |  |
| 7x7 c=6 | probcut-v1 | 0/30 (0.00-0.11) | > cap / > cap / > cap | 95.8 | 3.0 | 0/30 | 1:30 | 0.43 |
| 7x7 c=6 | probcut-v2 | 0/30 (0.00-0.11) | > cap / > cap / > cap | 92.0 | 2.9 | 0/30 | 1:30 | 0.13 |
| 8x8 c=10 | scb-gacolor_ac3_random | 19/30 (0.46-0.78) | 363,902 / 505,364 / > cap | 17310.7 | 597.8 |  |  |  |
| 8x8 c=10 | hp-dfs | 19/30 (0.46-0.78) | 360,947 / 509,862 / > cap | 698.6 | 20.2 |  |  |  |
| 8x8 c=10 | fc-dfs | 19/30 (0.46-0.78) | 360,829 / 508,613 / > cap | 44.0 | 1.6 |  |  |  |
| 8x8 c=10 | fc-luby | 16/30 (0.36-0.70) | 306,878 / 820,909 / > cap | 75.2 | 1.8 |  |  |  |
| 8x8 c=10 | probcut-v1 | 13/30 (0.27-0.61) | 395,888 / > cap / > cap | 64.2 | 1.6 | 13/30 | 1:30 | 0.30 |
| 8x8 c=10 | probcut-v2 | 17/30 (0.39-0.73) | 408,107 / 550,557 / > cap | 55.6 | 1.6 | 17/30 | 1:30 | 0.13 |

| rung | candidate | baseline | median ratio | 95% CI log-ratio | one-sided p | label (unadj.) | Holm label |
|---|---|---|---|---|---|---|---|
| 6x6 | probcut-v1 | scb-gacolor_ac3_random | 109.028 | [+4.10, +5.17] | 1.0000 | NO-GAIN | NO-GAIN |
| 6x6 | probcut-v1 | fc-dfs | 2.550 | [+0.45, +1.33] | 0.9996 | NO-GAIN |  |
| 6x6 | probcut-v1 | fc-luby | 1.789 | [+0.28, +1.02] | 0.9797 | NO-GAIN |  |
| 6x6 | probcut-v1 | hp-dfs | 2.270 | [+0.37, +1.25] | 0.9987 | NO-GAIN |  |
| 6x6 | probcut-v2 | scb-gacolor_ac3_random | 34.163 | [+3.13, +4.50] | 1.0000 | NO-GAIN | NO-GAIN |
| 6x6 | probcut-v2 | fc-dfs | 1.063 | [+0.03, +0.11] | 0.9981 | NO-GAIN |  |
| 6x6 | probcut-v2 | fc-luby | 0.755 | [-0.98, +0.20] | 0.2799 | NO-GAIN |  |
| 6x6 | probcut-v2 | hp-dfs | 0.979 | [-0.10, +0.02] | 0.0917 | NO-GAIN |  |
| 6x6 | fc-dfs | scb-gacolor_ac3_random | 30.555 | [+3.18, +4.08] | 1.0000 | NO-GAIN |  |
| 6x6 | fc-dfs | fc-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 6x6 | fc-dfs | fc-luby | 0.589 | [-1.16, +0.14] | 0.0913 | NO-GAIN |  |
| 6x6 | fc-dfs | hp-dfs | 0.920 | [-0.09, -0.07] | 0.0000 | GAIN |  |
| 6x6 | fc-luby | scb-gacolor_ac3_random | 70.160 | [+3.50, +4.55] | 1.0000 | NO-GAIN |  |
| 6x6 | fc-luby | fc-dfs | 2.437 | [-0.00, +1.20] | 0.9529 | NO-GAIN |  |
| 6x6 | fc-luby | fc-luby | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 6x6 | fc-luby | hp-dfs | 2.265 | [-0.08, +1.11] | 0.9529 | NO-GAIN |  |
| 7x7 | probcut-v1 | scb-gacolor_ac3_random | inf | [+0.00, inf] | 1.0000 | NO-GAIN | NO-GAIN |
| 7x7 | probcut-v1 | fc-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | probcut-v1 | fc-luby | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | probcut-v1 | hp-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | probcut-v2 | scb-gacolor_ac3_random | inf | [+0.00, inf] | 1.0000 | NO-GAIN | NO-GAIN |
| 7x7 | probcut-v2 | fc-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | probcut-v2 | fc-luby | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | probcut-v2 | hp-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | fc-dfs | scb-gacolor_ac3_random | inf | [+0.00, inf] | 1.0000 | NO-GAIN |  |
| 7x7 | fc-dfs | fc-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | fc-dfs | fc-luby | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | fc-dfs | hp-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | fc-luby | scb-gacolor_ac3_random | inf | [+0.00, inf] | 1.0000 | NO-GAIN |  |
| 7x7 | fc-luby | fc-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | fc-luby | fc-luby | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 7x7 | fc-luby | hp-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 8x8 | probcut-v1 | scb-gacolor_ac3_random | 1.000 | [+0.00, inf] | 0.9805 | NO-GAIN | NO-GAIN |
| 8x8 | probcut-v1 | fc-dfs | 1.000 | [-0.06, +0.00] | 0.9031 | NO-GAIN |  |
| 8x8 | probcut-v1 | fc-luby | 1.000 | [+0.00, inf] | 0.9827 | NO-GAIN |  |
| 8x8 | probcut-v1 | hp-dfs | 1.000 | [-0.07, +0.00] | 0.9031 | NO-GAIN |  |
| 8x8 | probcut-v2 | scb-gacolor_ac3_random | 1.000 | [-0.26, +0.52] | 0.8175 | NO-GAIN | NO-GAIN |
| 8x8 | probcut-v2 | fc-dfs | 1.000 | [+0.00, +0.04] | 0.9998 | NO-GAIN |  |
| 8x8 | probcut-v2 | fc-luby | 1.000 | [-0.26, +0.93] | 0.9055 | NO-GAIN |  |
| 8x8 | probcut-v2 | hp-dfs | 1.000 | [+0.00, +0.04] | 0.9998 | NO-GAIN |  |
| 8x8 | fc-dfs | scb-gacolor_ac3_random | 1.000 | [-0.34, +0.27] | 0.5770 | NO-GAIN |  |
| 8x8 | fc-dfs | fc-dfs | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 8x8 | fc-dfs | fc-luby | 1.000 | [-0.53, +0.35] | 0.8232 | NO-GAIN |  |
| 8x8 | fc-dfs | hp-dfs | 0.999 | [-0.00, +0.00] | 0.0955 | NO-GAIN |  |
| 8x8 | fc-luby | scb-gacolor_ac3_random | 1.000 | [+0.00, +1.00] | 0.9812 | NO-GAIN |  |
| 8x8 | fc-luby | fc-dfs | 1.000 | [-0.19, +1.08] | 0.9015 | NO-GAIN |  |
| 8x8 | fc-luby | fc-luby | 1.000 | [+0.00, +0.00] | 1.0000 | NO-GAIN |  |
| 8x8 | fc-luby | hp-dfs | 1.000 | [-0.19, +1.08] | 0.9015 | NO-GAIN |  |

Kill evaluation: {"stage1": {"auc_gain": true, "regression_r2": false, "false_cut": false, "fires": true}, "stage2_cost": {"fires": true, "v1_over_better_baseline_km_median": {"7": Infinity, "8": Infinity}}, "stage2_first_pass": {"fires": true, "v1_first_pass_solved_R8": 13}, "nogain_all_rungs": true, "r8_nogain_both": true, "r8_pass": false, "outcome": "killed"}
