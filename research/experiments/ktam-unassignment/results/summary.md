### R6

| arm | solved within E /30 | Wilson 95% | KM q25 / median / q75 events | solved within 1e6 nodes | KM median nodes | removals (total) | removed = planted | placed = planted | wall median ms | wall total s |
|---|---|---|---|---|---|---|---|---|---|---|
| B1 | 30 | 0.89 to 1.00 | 1,204 / 1,866 / 3,844 | 30 | 951 | n/a | n/a | n/a | 6 | 0.2 |
| B2 | 30 | 0.89 to 1.00 | 55,772 / 83,610 / 125,076 | 30 | 41,823 | n/a | n/a | n/a | 19 | 0.6 |
| B3 | 19 | 0.46 to 0.78 | 202,290 / 565,948 / censored | 24 | 282,992 | 13,557,588 | 0.128 | 0.128 | 297 | 12.3 |
| V1 | 23 | 0.59 to 0.88 | 154,016 / 481,968 / 814,676 | 27 | 241,002 | 9,618,001 | 0.085 | 0.085 | 267 | 10.1 |
| V2 | 27 | 0.74 to 0.97 | 51,300 / 221,284 / 620,100 | 30 | 110,660 | 5,939,624 | 0.068 | 0.068 | 88 | 4.2 |

| comparison (events, G1) | variant | median ratio | 95% CI of median ln ratio | p one-sided | Holm label | unadjusted label |
|---|---|---|---|---|---|---|
| vs B1 (B1) | V1 | 186.345 | 4.554 to 6.536 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1 (B1) | V2 | 100.350 | 3.295 to 5.493 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1orB3 (B1) | V1 | 186.345 | 4.554 to 6.536 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1orB3 (B1) | V2 | 100.350 | 3.295 to 5.493 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B3 (B3) | V1 | 0.880 | -0.730 to 0.000 | 0.2911 | NO-GAIN | NO-GAIN |
| vs B3 (B3) | V2 | 0.430 | -1.398 to -0.332 | 0.0042 | PASS | PASS |
| vs B2 (B2) | V1 | 7.644 | 0.976 to 2.860 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B2 (B2) | V2 | 2.887 | 0.388 to 1.535 | 0.9986 | NO-GAIN | NO-GAIN |

Best variant: V2; vs B3 median ratio 0.430, p = 0.0042.
Kill terms: `{"both_no_gain_vs_B1": true, "best_solved": 27, "max_baseline_solved": 30, "best_le_max_plus_3": true, "b3_within_2": false, "best_not_sig_vs_B3": false}`
Baseline reproduction: `{"B1": 0, "B2": 0}` mismatching rows.

### R7

| arm | solved within E /30 | Wilson 95% | KM q25 / median / q75 events | solved within 1e6 nodes | KM median nodes | removals (total) | removed = planted | placed = planted | wall median ms | wall total s |
|---|---|---|---|---|---|---|---|---|---|---|
| B1 | 10 | 0.19 to 0.51 | 544,513 / censored / censored | 16 | 921,773 | n/a | n/a | n/a | 6672 | 160.3 |
| B2 | 0 | 0.00 to 0.11 | censored / censored / censored | 0 | censored | n/a | n/a | n/a | 476 | 14.9 |
| B3 | 0 | 0.00 to 0.11 | censored / censored / censored | 0 | censored | 29,999,574 | 0.099 | 0.099 | 1007 | 32.2 |
| V1 | 0 | 0.00 to 0.11 | censored / censored / censored | 0 | censored | 29,999,390 | 0.060 | 0.060 | 1305 | 40.5 |
| V2 | 0 | 0.00 to 0.11 | censored / censored / censored | 0 | censored | 29,999,064 | 0.046 | 0.046 | 1012 | 32.6 |

| comparison (events, G1) | variant | median ratio | 95% CI of median ln ratio | p one-sided | Holm label | unadjusted label |
|---|---|---|---|---|---|---|
| vs B1 (B1) | V1 | 1.000 | 0.000 to +inf | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1 (B1) | V2 | 1.000 | 0.000 to +inf | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1orB3 (B1) | V1 | 1.000 | 0.000 to +inf | 1.0000 | NO-GAIN | NO-GAIN |
| vs B1orB3 (B1) | V2 | 1.000 | 0.000 to +inf | 1.0000 | NO-GAIN | NO-GAIN |
| vs B3 (B3) | V1 | 1.000 | 0.000 to 0.000 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B3 (B3) | V2 | 1.000 | 0.000 to 0.000 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B2 (B2) | V1 | 1.000 | 0.000 to 0.000 | 1.0000 | NO-GAIN | NO-GAIN |
| vs B2 (B2) | V2 | 1.000 | 0.000 to 0.000 | 1.0000 | NO-GAIN | NO-GAIN |

Best variant: V1; vs B3 median ratio 1.000, p = 1.0000.
Kill terms: `{"both_no_gain_vs_B1": true, "best_solved": 0, "max_baseline_solved": 10, "best_le_max_plus_3": true, "b3_within_2": true, "best_not_sig_vs_B3": true}`
Baseline reproduction: `{"B1": 0, "B2": 0}` mismatching rows.

R8 triggered: False. Method killed: True. Mechanism killed: False. Self-assessed outcome: **killed**.
