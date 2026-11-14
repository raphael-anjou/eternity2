| mode | pairs | solved, mode | median evaluations, mode | solved, baseline | median evaluations, baseline |
|---|---|---|---|---|---|
| l2 | 16 | 8 | 10,415 | 8 | 10,415 |
| conc | 6 | 2 | over 12,000 | 5 | 5,002 |
| eq | 4 | 0 | over 12,000 | 4 | 4,493 |
| l2@i100 | 5 | 5 | 5,510 | 5 | 4,294 |
| l2@i30 | 6 | 6 | 2,881 | 5 | 5,002 |
| l2@p0 | 14 | 6 | over 12,000 | 7 | 9,160 |
| lin | 5 | 0 | over 12,000 | 5 | 4,294 |
| none | 5 | 0 | over 12,000 | 5 | 4,294 |
| w3 | 5 | 0 | over 12,000 | 5 | 4,294 |

| board (solutions) | solutions left by the hint | placements deleted | starts | solved, no hint | evaluations, no hint | solved, hint | evaluations, hint |
|---|---|---|---|---|---|---|---|
| seed 0 (36) | 14 | 169 | 3 | 1 | 6,320, none, none | 0 | none, none, none |
| seed 1 (6) | 3 | 162 | 3 | 1 | 3,277, none, none | 2 | 5,710, 5,833, none |
| seed 2 (8) | 1 | 163 | 3 | 2 | 3,270, none, 5,285 | 1 | 11,874, none, none |
| seed 3 (403) | 14 | 147 | 3 | 2 | 5,709, 6,184, none | 3 | 5,222, 5,206, 5,617 |
| seed 4 (2) | 1 | 155 | 2 | 1 | 4,294, none | 0 | none, none |

baseline at the solution: 8 runs; rank_1e2 [30, 30, 31, 32, 32, 32, 32, 32] rho [106, 106, 179, 512, 665, 1125, 1125, 9173] mass off [0.0017, 0.0078, 0.115, 0.1527, 0.323, 0.4335, 0.4693, 0.6461] viol [0.003814, 0.003924, 0.004704, 0.006012, 0.006427, 0.006525, 0.007977, 0.01035] evals [3270, 3277, 4294, 5285, 5709, 6184, 6320, 8830]
baseline all: [(0, 0, 6320), (0, 1, None), (0, 2, None), (0, 3, 8830), (1, 0, 3277), (1, 1, None), (1, 2, None), (2, 0, 3270), (2, 1, None), (2, 2, 5285), (3, 0, 5709), (3, 1, 6184), (3, 2, None), (4, 0, 4294), (4, 1, None), (4, 2, None)]

| board | objective | scale s | iterations run | first valid rounding | mass off under 1e-2 / 1e-5 | end residual | rank of Z | dual rank | eigenvalues within 1e-6 | free pairs: primal positive / multiplier over 1e-8 / neither |
|---|---|---|---|---|---|---|---|---|---|---|
| seed 1 (6) | none | 1 | 3,790 | never | 2900 / 3000 | 9e-09 | 6 | 427 | 0 | 1,482 / 83,857 / 1,226 |
| seed 1 (6) | linear, tau 0.1, draw 5 | 1 | 6,000 | 1750 | 3750 / 3800 | 3e-06 | 1 | 432 | 0 | 300 / 83,466 / 2,799 |
| seed 1 (6) | norm, rho 300 | 1 | 6,000 | never | 4225 / 4400 | 1e-04 | 6 | 427 | 0 | 1,482 / 83,432 / 1,651 |
| seed 1 (6) | none | 3 | 4,635 | never | 2850 / 2900 | 1e-08 | 6 | 427 | 0 | 1,482 / 84,235 / 848 |
| seed 1 (6) | none | 0.33 | 4,078 | never | 3625 / 3675 | 1e-08 | 6 | 427 | 0 | 1,482 / 81,469 / 3,614 |
| seed 2 (8) | none | 1 | 6,000 | never | 5450 / 5600 | 5e-07 | 8 | 425 | 0 | 1,725 / 83,939 / 565 |
| seed 2 (8) | linear, tau 0.1, draw 5 | 1 | 6,000 | 1175 | 3100 / 3150 | 1e-03 | 2 | 431 | 0 | 390 / 83,825 / 2,014 |
| seed 1 (6) | none | 10 | 5,205 | never | 2900 / 3000 | 1e-08 | 6 | 427 | 0 | 1,482 / 84,291 / 792 |
| seed 1 (6) | linear, tau 0.1, draw 6 | 1 | 6,000 | never | 5050 / 5100 | 6e-04 | 3 | 430 | 0 | 873 / 83,959 / 1,733 |
