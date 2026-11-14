Node identity gate: 450 instances checked, 0 mismatches.

Soundness gate: 7524 live states on 90 instances.

| oracle | live checks | violations |
|---|---|---|
| a1 | 7524 | 0 |
| a1_rows | 7524 | 0 |
| a2 | 7524 | 0 |
| a2_rows | 7524 | 0 |
| a3 | 7524 | 0 |
| a3_rows | 7524 | 0 |
| b_dom_r<=2 | 7524 | 0 |
| b_dom_r<=3 | 7524 | 0 |
| b_dom_r<=4 | 7524 | 0 |
| b_pool_r<=2 | 7524 | 0 |
| b_pool_r<=3 | 7524 | 0 |
| b_pool_r<=4 | 7524 | 0 |
| d_cut | 7524 | 0 |
| d_poac | 7524 | 0 |
| d_poac_mrv | 7524 | 0 |
| d_rand | 7524 | 0 |
| e | 7524 | 0 |
| pocket | 7524 | 0 |

Fuzz against brute force:

| check | cases | violations |
|---|---|---|
| band_vs_naive | 41913 | 0 |
| cavity_vs_naive | 80320 | 0 |
| colour_vs_naive | 11903 | 0 |
| oracles_vs_exact | 15005 | 0 |
| prop_equal | 16539 | 0 |
| states_dead | 594 | 0 |
| states_live | 14411 | 0 |

## R6

Instances 30 ({'solved': 30, 'censored': 0, 'exhausted': 0}), with dead nodes 30, dead nodes 13966, sampled 5335.

| oracle | instances | weighted share [95% CI] | unweighted share [95% CI] |
|---|---|---|---|
| b_pool_r<=2 | 30 | 0.001 [0.000, 0.001] | 0.003 [0.001, 0.005] |
| b_pool_r<=3 | 30 | 0.017 [0.010, 0.027] | 0.054 [0.038, 0.075] |
| b_pool_r<=4 | 30 | 0.225 [0.195, 0.256] | 0.510 [0.458, 0.561] |
| b_dom_r<=2 | 30 | 0.006 [0.004, 0.009] | 0.021 [0.013, 0.030] |
| b_dom_r<=3 | 30 | 0.108 [0.090, 0.129] | 0.279 [0.244, 0.316] |
| b_dom_r<=4 | 30 | 0.508 [0.466, 0.549] | 0.833 [0.806, 0.858] |
| a1 | 30 | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| a1_rows | 30 | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| a2 | 30 | 0.002 [0.001, 0.004] | 0.006 [0.003, 0.010] |
| a2_rows | 30 | 0.001 [0.000, 0.002] | 0.003 [0.001, 0.006] |
| a3 | 30 | 0.011 [0.007, 0.017] | 0.032 [0.022, 0.043] |
| a3_rows | 30 | 0.008 [0.005, 0.014] | 0.024 [0.016, 0.033] |
| e | 30 | 0.019 [0.013, 0.026] | 0.058 [0.042, 0.073] |
| d_cut | 30 | 0.245 [0.218, 0.274] | 0.553 [0.527, 0.579] |
| d_rand | 30 | 0.251 [0.223, 0.283] | 0.544 [0.522, 0.567] |
| d_poac | 30 | 0.144 [0.121, 0.178] | 0.428 [0.402, 0.458] |
| d_poac_mrv | 30 | 0.037 [0.032, 0.045] | 0.171 [0.157, 0.186] |
| pocket | 30 | 0.070 [0.057, 0.084] | 0.199 [0.165, 0.235] |

| work | values |
|---|---|
| b_pool | median_work 108990, median_ns 1037708, median_ns_over_subtree_ns 7.552 |
| b_dom | median_work 31884, median_ns 387416, median_ns_over_subtree_ns 2.839 |
| a1 | median_work 1614, median_ns 29459, median_ns_over_subtree_ns 0.248 |
| a2 | median_work 14879, median_ns 132417, median_ns_over_subtree_ns 1.021 |
| a3 | median_work 50024, median_ns 302750, median_ns_over_subtree_ns 2.453 |
| e | median_work 2602, median_ns 56792, median_ns_over_subtree_ns 0.638 |
| pocket | median_work 4474, median_ns 40666, median_ns_over_subtree_ns 0.396 |
| d_cut | median_calls 6, median_calls_over_subtree 0.667, key_found_share 1.000 |
| d_rand | median_calls 10, median_calls_over_subtree 1.200, key_found_share 1.000 |
| d_poac | median_calls 4, median_calls_over_subtree 0.571, key_found_share 1.000 |
| d_poac_mrv | median_calls 1, median_calls_over_subtree 0.167, key_found_share 1.000 |
| d_ns_all_arms | 267083 |
| b_pool_budget_outs | 829 |
| b_dom_budget_outs | 786 |
| e_overflows | 0 |
| pocket_overflows | 0 |

B-pool key recurrence among refuted sampled nodes (weighted unless stated):

| statistic | mean [95% CI] | instances |
|---|---|---|
| raw_rec_weighted | 0.128 [0.092, 0.170] | 30 |
| raw_rec_unweighted | 0.141 [0.103, 0.185] | 30 |
| can_rec_weighted | 0.130 [0.094, 0.171] | 30 |
| can_rec_unweighted | 0.143 [0.104, 0.187] | 30 |
| raw_sound_weighted | 0.045 [0.020, 0.076] | 30 |
| raw_sound_unweighted | 0.050 [0.022, 0.083] | 30 |
| can_sound_weighted | 0.045 [0.020, 0.076] | 30 |
| can_sound_unweighted | 0.050 [0.022, 0.083] | 30 |
| raw_rec_multiplicity_median | 0.000 | |

b_pool: weighted median r = inf; weighted distribution r=2: 0.001, r=3: 0.016, r=4: 0.208, r=inf: 0.775

b_dom: weighted median r = 4; weighted distribution r=2: 0.006, r=3: 0.102, r=4: 0.400, r=inf: 0.492

Pocket local-core rate (s >= 200): median 0.000 over 19 instances; recurrence among sampled refuted pockets 0.049.

Weighted share by depth band (pooled):

- b_pool_r<=3: early 0.007, late 0.640, mid 0.042
- a2: early 0.001, late 0.107, mid 0.007
- e: early 0.007, late 0.165, mid 0.053
- d_cut: early 0.130, late 1.000, mid 0.547
- pocket: early 0.008, late 1.000, mid 0.231

## R7

Instances 30 ({'solved': 23, 'censored': 7, 'exhausted': 0}), with dead nodes 30, dead nodes 11191605, sampled 6000.

| oracle | instances | weighted share [95% CI] | unweighted share [95% CI] |
|---|---|---|---|
| b_pool_r<=2 | 30 | 0.001 [0.000, 0.001] | 0.004 [0.002, 0.006] |
| b_pool_r<=3 | 30 | 0.008 [0.007, 0.010] | 0.057 [0.048, 0.066] |
| b_pool_r<=4 | 30 | 0.061 [0.056, 0.068] | 0.358 [0.331, 0.384] |
| b_dom_r<=2 | 30 | 0.006 [0.004, 0.007] | 0.034 [0.027, 0.041] |
| b_dom_r<=3 | 30 | 0.040 [0.036, 0.044] | 0.240 [0.218, 0.262] |
| b_dom_r<=4 | 30 | 0.157 [0.146, 0.168] | 0.674 [0.649, 0.696] |
| a1 | 30 | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| a1_rows | 30 | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| a2 | 30 | 0.002 [0.001, 0.003] | 0.013 [0.009, 0.017] |
| a2_rows | 30 | 0.001 [0.001, 0.002] | 0.010 [0.006, 0.013] |
| a3 | 30 | 0.008 [0.006, 0.009] | 0.049 [0.042, 0.057] |
| a3_rows | 30 | 0.006 [0.004, 0.007] | 0.036 [0.029, 0.043] |
| e | 30 | 0.017 [0.014, 0.020] | 0.110 [0.095, 0.126] |
| d_cut | 30 | 0.103 [0.096, 0.111] | 0.504 [0.482, 0.525] |
| d_rand | 30 | 0.096 [0.089, 0.106] | 0.480 [0.462, 0.496] |
| d_poac | 30 | 0.063 [0.058, 0.068] | 0.415 [0.396, 0.435] |
| d_poac_mrv | 30 | 0.016 [0.015, 0.017] | 0.167 [0.156, 0.179] |
| pocket | 30 | 0.035 [0.032, 0.039] | 0.225 [0.204, 0.246] |

| work | values |
|---|---|
| b_pool | median_work 206906.500, median_ns 1996125.500, median_ns_over_subtree_ns 3.841 |
| b_dom | median_work 138814.500, median_ns 1325916.500, median_ns_over_subtree_ns 1.147 |
| a1 | median_work 3298.500, median_ns 70333.000, median_ns_over_subtree_ns 0.203 |
| a2 | median_work 52563.000, median_ns 451646.000, median_ns_over_subtree_ns 0.834 |
| a3 | median_work 282283.500, median_ns 1617041.500, median_ns_over_subtree_ns 2.030 |
| e | median_work 20480.000, median_ns 398562.500, median_ns_over_subtree_ns 0.716 |
| pocket | median_work 10123.500, median_ns 111270.500, median_ns_over_subtree_ns 0.388 |
| d_cut | median_calls 6.000, median_calls_over_subtree 0.500, key_found_share 1.000 |
| d_rand | median_calls 15.000, median_calls_over_subtree 1.000, key_found_share 1.000 |
| d_poac | median_calls 4.000, median_calls_over_subtree 0.432, key_found_share 1.000 |
| d_poac_mrv | median_calls 2.000, median_calls_over_subtree 0.118, key_found_share 1.000 |
| d_ns_all_arms | 631271.000 |
| b_pool_budget_outs | 212 |
| b_dom_budget_outs | 254 |
| e_overflows | 0 |
| pocket_overflows | 0 |

B-pool key recurrence among refuted sampled nodes (weighted unless stated):

| statistic | mean [95% CI] | instances |
|---|---|---|
| raw_rec_weighted | 0.459 [0.413, 0.505] | 30 |
| raw_rec_unweighted | 0.437 [0.393, 0.480] | 30 |
| can_rec_weighted | 0.463 [0.418, 0.508] | 30 |
| can_rec_unweighted | 0.442 [0.399, 0.484] | 30 |
| raw_sound_weighted | 0.135 [0.101, 0.169] | 30 |
| raw_sound_unweighted | 0.128 [0.100, 0.157] | 30 |
| can_sound_weighted | 0.135 [0.102, 0.170] | 30 |
| can_sound_unweighted | 0.128 [0.101, 0.157] | 30 |
| raw_rec_multiplicity_median | 0 | |

b_pool: weighted median r = inf; weighted distribution r=2: 0.001, r=3: 0.008, r=4: 0.053, r=inf: 0.939

b_dom: weighted median r = inf; weighted distribution r=2: 0.006, r=3: 0.034, r=4: 0.117, r=inf: 0.843

Pocket local-core rate (s >= 200): median 0.000 over 30 instances; recurrence among sampled refuted pockets 0.009.

Weighted share by depth band (pooled):

- b_pool_r<=3: early 0.000, late 0.454, mid 0.023
- a2: early 0.000, late 0.093, mid 0.005
- e: early 0.000, late 0.586, mid 0.048
- d_cut: early 0.009, late 0.990, mid 0.277
- pocket: early 0.000, late 1.000, mid 0.100

## N7

Instances 30 ({'solved': 30, 'censored': 0, 'exhausted': 0}), with dead nodes 30, dead nodes 78220, sampled 5912.

| oracle | instances | weighted share [95% CI] | unweighted share [95% CI] |
|---|---|---|---|
| b_pool_r<=2 | 30 | 0.001 [0.000, 0.001] | 0.003 [0.001, 0.004] |
| b_pool_r<=3 | 30 | 0.014 [0.011, 0.017] | 0.057 [0.047, 0.068] |
| b_pool_r<=4 | 30 | 0.088 [0.076, 0.100] | 0.305 [0.275, 0.336] |
| b_dom_r<=2 | 30 | 0.011 [0.009, 0.014] | 0.041 [0.034, 0.049] |
| b_dom_r<=3 | 30 | 0.088 [0.078, 0.098] | 0.300 [0.275, 0.325] |
| b_dom_r<=4 | 30 | 0.249 [0.231, 0.267] | 0.695 [0.675, 0.715] |
| a1 | 30 | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| a1_rows | 30 | 0.000 [0.000, 0.000] | 0.000 [0.000, 0.000] |
| a2 | 30 | 0.011 [0.009, 0.014] | 0.043 [0.034, 0.052] |
| a2_rows | 30 | 0.007 [0.005, 0.008] | 0.026 [0.020, 0.032] |
| a3 | 30 | 0.045 [0.040, 0.050] | 0.161 [0.148, 0.174] |
| a3_rows | 30 | 0.033 [0.030, 0.036] | 0.124 [0.112, 0.137] |
| e | 30 | 0.208 [0.192, 0.224] | 0.602 [0.572, 0.632] |
| d_cut | 30 | 0.184 [0.168, 0.203] | 0.557 [0.531, 0.585] |
| d_rand | 30 | 0.164 [0.154, 0.176] | 0.504 [0.488, 0.521] |
| d_poac | 30 | 0.149 [0.142, 0.156] | 0.546 [0.528, 0.563] |
| d_poac_mrv | 30 | 0.038 [0.035, 0.041] | 0.221 [0.208, 0.234] |
| pocket | 30 | 0.056 [0.048, 0.064] | 0.210 [0.183, 0.237] |

| work | values |
|---|---|
| b_pool | median_work 311621.000, median_ns 3025375.500, median_ns_over_subtree_ns 9.447 |
| b_dom | median_work 90338.000, median_ns 978791.500, median_ns_over_subtree_ns 2.200 |
| a1 | median_work 4665.000, median_ns 97896.000, median_ns_over_subtree_ns 0.375 |
| a2 | median_work 81802.000, median_ns 661520.500, median_ns_over_subtree_ns 1.941 |
| a3 | median_work 439281.500, median_ns 2186750.000, median_ns_over_subtree_ns 5.893 |
| e | median_work 3302.500, median_ns 101104.000, median_ns_over_subtree_ns 0.575 |
| pocket | median_work 11352.000, median_ns 105750.000, median_ns_over_subtree_ns 0.515 |
| d_cut | median_calls 6.000, median_calls_over_subtree 1.000, key_found_share 1.000 |
| d_rand | median_calls 21.000, median_calls_over_subtree 2.182, key_found_share 1.000 |
| d_poac | median_calls 4.000, median_calls_over_subtree 0.667, key_found_share 1.000 |
| d_poac_mrv | median_calls 2.000, median_calls_over_subtree 0.200, key_found_share 1.000 |
| d_ns_all_arms | 809208.500 |
| b_pool_budget_outs | 779 |
| b_dom_budget_outs | 753 |
| e_overflows | 0 |
| pocket_overflows | 0 |

B-pool key recurrence among refuted sampled nodes (weighted unless stated):

| statistic | mean [95% CI] | instances |
|---|---|---|
| raw_rec_weighted | 0.484 [0.426, 0.543] | 30 |
| raw_rec_unweighted | 0.482 [0.427, 0.537] | 30 |
| can_rec_weighted | 0.514 [0.460, 0.569] | 30 |
| can_rec_unweighted | 0.502 [0.448, 0.555] | 30 |
| raw_sound_weighted | 0.146 [0.092, 0.210] | 30 |
| raw_sound_unweighted | 0.139 [0.090, 0.196] | 30 |
| can_sound_weighted | 0.163 [0.109, 0.225] | 30 |
| can_sound_unweighted | 0.152 [0.102, 0.208] | 30 |
| raw_rec_multiplicity_median | 0.000 | |

b_pool: weighted median r = inf; weighted distribution r=2: 0.001, r=3: 0.014, r=4: 0.074, r=inf: 0.912

b_dom: weighted median r = inf; weighted distribution r=2: 0.011, r=3: 0.076, r=4: 0.161, r=inf: 0.751

Pocket local-core rate (s >= 200): median 0.000 over 28 instances; recurrence among sampled refuted pockets 0.035.

Weighted share by depth band (pooled):

- b_pool_r<=3: early 0.011, mid 0.180
- a2: early 0.009, mid 0.131
- e: early 0.194, mid 0.932
- d_cut: early 0.171, mid 0.876
- pocket: early 0.048, mid 0.482

## Verdicts

```
{
 "T2_GC": {
  "K0a": {
   "stat": 0.008264223902508486,
   "threshold": "<0.30",
   "fires": true
  },
  "K0b": {
   "stat": 0.45946211379218177,
   "threshold": "<0.10",
   "fires": false
  },
  "K0c": {
   "R7_median_r": "inf",
   "N7_median_r": "inf",
   "fires": true
  },
  "killed": true
 },
 "T5_TW": {
  "census_kill": {
   "R6": 0.002265731715090099,
   "R7": 0.0018447290534346673,
   "threshold": "<0.05 at both",
   "fires": true
  },
  "mechanism_kill": {
   "R6_a1": 0.0,
   "R6_a2_ci": [
    0.0009827307810898942,
    0.003807579290351819
   ],
   "R7_a1": 0.0,
   "R7_a2_ci": [
    0.0012087039674867996,
    0.002547258088427639
   ],
   "fires": false
  },
  "killed": true
 },
 "T3_CO": {
  "K2": {
   "R6_cut": 0.2448661225153714,
   "R6_rand": 0.25055914795592227,
   "R7_cut": 0.10302909044533277,
   "R7_rand": 0.09636182543603604,
   "fires": true
  },
  "K3": {
   "R7_cut": 0.10302909044533277,
   "R7_poac": 0.06293431434190017,
   "fires": false
  },
  "K1": "not decided by the census",
  "killed": true
 },
 "ST": {
  "local_core_rate_R7_median": 0.0,
  "threshold": "<0.10",
  "fires": true,
  "projected_saving": "not decided by the census",
  "killed": true
 },
 "OC": "no kill decided by the census; E floor at R7 = 0.0171"
}
```
