## Per rung and arm (run seed 1)

| rung | arm | solved/N | Wilson 95% | KM q25 | KM median | KM q75 | median wall ms | total wall s | time-cens | fallback frac | revisit frac | cycled runs |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| R6 6x6 c=5 | scb-gacolor_ac3_random | 30/30 | 0.89-1.00 | 620 | 951 | 1,940 | 7 | 0.3 | 0 | - | - | - |
| R6 6x6 c=5 | bb-mrv | 30/30 | 0.89-1.00 | 317 | 460 | 1,092 | 7 | 0.3 | 0 | - | - | - |
| R6 6x6 c=5 | bj-blocker | 30/30 | 0.89-1.00 | 2,067 | 4,334 | 12,008 | 182 | 11.5 | 0 | 0.244 | 0.365 | 0 |
| R6 6x6 c=5 | bj-random | 30/30 | 0.89-1.00 | 738 | 3,868 | 14,733 | 156 | 63.3 | 0 | 0.039 | 0.396 | 0 |
| R7 7x7 c=6 | scb-gacolor_ac3_random | 16/30 | 0.36-0.70 | 272,281 | 921,773 | inf | 7,745 | 181.0 | 0 | - | - | - |
| R7 7x7 c=6 | bb-mrv | 23/30 | 0.59-0.88 | 253,491 | 588,305 | 976,206 | 17,726 | 471.0 | 0 | - | - | - |
| R7 7x7 c=6 | bj-blocker | 5/30 | 0.07-0.34 | inf | inf | inf | 71,210 | 1,893.4 | 0 | 0.285 | 0.257 | 5 |
| R7 7x7 c=6 | bj-random | 2/30 | 0.02-0.21 | inf | inf | inf | 75,691 | 2,149.6 | 0 | 0.034 | 0.130 | 0 |

## Paired comparisons (G1 form, run seed 1)

| rung | candidate | baseline | pairs | node ratio | 95% CI | G1 | cand-only | base-only | both-cens | wall ratio | wall 95% CI |
|---|---|---|---|---|---|---|---|---|---|---|---|
| R6 6x6 c=5 | bj-blocker | scb-gacolor_ac3_random | 30 | 4.683 | [2.403, 6.547] | fail | 0 | 0 | 0 | 32.271 | [17.508, 43.995] |
| R6 6x6 c=5 | bj-blocker | bb-mrv | 30 | 9.177 | [5.394, 15.204] | fail | 0 | 0 | 0 | 24.503 | [14.495, 38.286] |
| R6 6x6 c=5 | bj-random | scb-gacolor_ac3_random | 30 | 4.792 | [1.982, 6.239] | fail | 0 | 0 | 0 | 32.775 | [12.385, 50.041] |
| R6 6x6 c=5 | bj-random | bb-mrv | 30 | 6.953 | [2.718, 15.771] | fail | 0 | 0 | 0 | 19.174 | [7.339, 36.782] |
| R6 6x6 c=5 | bj-blocker | bj-random | 30 | 0.947 | [0.574, 3.670] | fail | 0 | 0 | 0 | 0.978 | [0.573, 2.981] |
| R6 6x6 c=5 | bb-mrv | scb-gacolor_ac3_random | 30 | 0.468 | [0.318, 0.895] | PASS | 0 | 0 | 0 | 1.168 | [0.858, 2.390] |
| R7 7x7 c=6 | bj-blocker | scb-gacolor_ac3_random | 30 | 1.000 | [1.000, inf] | fail | 1 | 12 | 13 | 9.009 | [8.070, 12.885] |
| R7 7x7 c=6 | bj-blocker | bb-mrv | 30 | inf | [3.813, inf] | fail | 0 | 18 | 7 | 3.961 | [2.964, 5.644] |
| R7 7x7 c=6 | bj-random | scb-gacolor_ac3_random | 30 | 3.173 | [1.000, inf] | fail | 0 | 14 | 14 | 9.620 | [8.775, 20.835] |
| R7 7x7 c=6 | bj-random | bb-mrv | 30 | inf | [inf, inf] | fail | 0 | 21 | 7 | 4.216 | [3.340, 7.993] |
| R7 7x7 c=6 | bj-blocker | bj-random | 30 | 1.000 | [1.000, 1.000] | fail | 4 | 1 | 24 | 0.971 | [0.930, 0.988] |
| R7 7x7 c=6 | bb-mrv | scb-gacolor_ac3_random | 30 | 0.976 | [0.636, 1.000] | fail | 9 | 2 | 5 | 2.863 | [2.038, 3.124] |

## Run seeds 1 to 3 (bj arms, solved/30)

| rung | arm | seed 1 | seed 2 | seed 3 | median |
|---|---|---|---|---|---|
| R6 6x6 c=5 | bj-blocker | 30 | 30 | 29 | 30 |
| R6 6x6 c=5 | bj-random | 30 | 30 | 29 | 30 |
| R7 7x7 c=6 | bj-blocker | 5 | 2 | 4 | 4 |
| R7 7x7 c=6 | bj-random | 2 | 4 | 3 | 3 |

## Verdicts

```
{
 "protocol_G1_R6_pass": false,
 "protocol_dropped_at_R6": true,
 "R7_gate_pass": false,
 "K1_A20_method_kill": true,
 "K1_inputs": {
  "G1_R6": false,
  "G1_R7": false,
  "blocker_R7_solved_runseed_median": 4,
  "blocker_R7_solved_seed1": 5,
  "scb_R7_solved": 16
 },
 "method_killed": true,
 "K2_mechanism_kill_R7": true,
 "K2_inputs": {
  "twin_R7_solved": 3,
  "blocker_R7_solved": 4,
  "blocker_vs_twin_ci_ln_upper": 0.0
 }
}
```
