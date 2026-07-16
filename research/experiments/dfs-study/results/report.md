# DFS study — grid report

170 runs · 10 corner-pinned variants · 60 s each · seed 1 · single core.

Score = matched edges of 480; break variants score 480 − #breaks. Throughput is search-nodes/s and is never compared across families.


## baseline

| variant | delta | breaks | mean | best | worst | max depth | median nps |
|:--|:--|:--|--:|--:|--:|--:|--:|
| NAIVE-CLEAN | the rawest depth-first backtracker: row-major, no heuristics, no breaks | strict | 376.8 | 382 | 374 | 208 | 32.4M |
| NAIVE-CODEGEN | same algorithm, a 16×16-specialised unrolled hot loop | strict | 372.1 | 378 | 365 | 216 | 44.9M |

## path

| variant | delta | breaks | mean | best | worst | max depth | median nps |
|:--|:--|:--|--:|--:|--:|--:|--:|
| ROWMAJOR | the row-major control (same as NAIVE-CLEAN, named for the path study) | strict | 376.6 | 382 | 374 | 208 | 31.5M |
| ROWMAJOR-BOTTOMUP | fill bottom-to-top instead of top-to-bottom (Blackwood's scan direction) | strict | 225.9 | 366 | 18 | 201 | 273K |
| SPIRAL-IN | fill the outer ring inward instead of row-major (the cloister spiral) | strict | 80.2 | 91 | 62 | 79 | 13.3M |
| SPIRAL-OUT | spiral from the centre outward instead of inward | strict | 24.5 | 34 | 15 | 31 | 20.1M |
| BORDER-FIRST | fill the whole border ring first, then the interior | strict | 66.6 | 85 | 62 | 77 | 365K |
| VERHAARD-COMB | a horizontal band then vertical teeth (Verhaard's COMB order) | strict | 357 | 360 | 350 | 197 | 23.3M |

## heuristic

| variant | delta | breaks | mean | best | worst | max depth | median nps |
|:--|:--|:--|--:|--:|--:|--:|--:|
| BORDER-MRV | choose the most-constrained empty cell dynamically (MRV) instead of a fixed order | strict | 324.1 | 341 | 300 | 194 | 6K |
| MRV-RARE | try pieces carrying globally-rare colours first (Selby/Riordan rarity) | strict | 323.7 | 341 | 296 | 194 | 7K |
| MRV-FC | add forward-checking: reject a placement that empties any neighbour's domain | strict | 322.2 | 339 | 303 | 193 | 7K |
| MRV-AC3 | extend the look-ahead to arc-consistency (AC-3) over the frontier | strict | 320.6 | 338 | 301 | 192 | 8K |
| MRV-GACOLOR | add Régin per-colour all-different reasoning on the remaining supply | strict | 320.6 | 338 | 301 | 192 | 8K |

## break

| variant | delta | breaks | mean | best | worst | max depth | median nps |
|:--|:--|:--|--:|--:|--:|--:|--:|
| BREAK-1 | allow ≤1 broken edge per cell on a depth schedule (Blackwood's ladder) | break (≤1/cell) | 431.3 | 435 | 427 | 245 | 4.6M |
| BREAK-2 | allow up to 2 broken edges at one cell (double-breaks the community 460s use) | break (≤2/cell) | 428.5 | 435 | 402 | 245 | 5.2M |
| VERHAARD-SLIP | Verhaard's interior edge-slip schedule instead of Blackwood's ladder | break (≤1/cell) | 399.3 | 430 | 378 | 243 | 2.3M |
| BLACKWOOD-COMB-BREAK | run the Blackwood break ladder on Verhaard's COMB fill order | break (≤1/cell) | 356.4 | 360 | 350 | 197 | 17.9M |

## community

| variant | delta | breaks | mean | best | worst | max depth | median nps |
|:--|:--|:--|--:|--:|--:|--:|--:|
| MCGAVIN-C | the community's own record engine (run as published, cited not re-run here) | break (≤1/cell) | cited | cited | cited | cited | cited |
| BLACKWOOD-CS | the community's own record engine (run as published, cited not re-run here) | break (≤1/cell) | cited | cited | cited | cited | cited |
