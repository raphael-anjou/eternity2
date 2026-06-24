# Complex Theory (Brendan Owen / Peter McGavin)

The site's live complex-theory estimator
([`web/src/engine-ts/complexity.ts`](../../../web/src/engine-ts/complexity.ts))
is a faithful, validated port of **Peter McGavin's reference C implementation**
of Brendan Owen's Complex Theory.

## Source

- **McGavin's post (with the attached C source and the published depth table):**
  <https://groups.io/g/eternity2/message/11197>
- **The C source itself:**
  <https://groups.io/g/eternity2/attachment/11197/0/complex_theory.c>
  (a copy is kept here at [`reference/complex_theory.c`](reference/complex_theory.c)
  for provenance — © Peter McGavin, 2024)
- **Brendan Owen's original theory:** groups.io messages
  [5197](https://groups.io/g/eternity2/message/5197) and
  [5209](https://groups.io/g/eternity2/message/5209).

## What it computes

Given a board state (which squares are occupied), the **expected number of ways
that state could be a valid partial solution**:

```
s2 = P · pm2 · pb2
```

- `P` — ways to place the occupied pieces: `P(corners)·P(borders)·P(middles)·4^middles`.
- `pm2` — probability the completed interior ("middle") joins are all colour-valid.
- `pb2` — probability the completed border joins are all valid.

`validm` / `validb` are recursive convolutions over the colour-type histograms
(`mt`, `bt`) — the number of ways to form the completed joins from same-colour
half-edges. See the C for the exact recurrences; our port mirrors them line for
line, evaluated in log10 (with log-sum-exp and `lgamma`) so the 10⁴⁷-scale
intermediates never overflow Float64 (the C uses GNU MP for the same reason).

## Validation

Compile and run the reference:

```sh
gcc -o complex_theory reference/complex_theory.c -lgmp -lm -O3
./complex_theory
```

Our TypeScript port reproduces McGavin's published table on the official 16×16
puzzle with the single I8 hint — every checkpoint matches:

| depth | num-solns | cum-solns |
|------:|----------:|----------:|
| 2     | 4.000     | 5.000     |
| 3     | 44.80     | 49.80     |
| 5     | 5341      | 5884      |
| 8     | 6.089×10⁶ | 6.745×10⁶ |
| 16    | 3.689×10¹⁴| 4.165×10¹⁴|
| 157   | 5.045×10⁴⁵| —         |
| **256** | **14,702** | **1.365×10⁴⁷** |

The edge-type histograms `bt`/`mt` are derived automatically from any chosen
puzzle's piece set (for the official puzzle this recovers McGavin's hard-coded
`bt = {12×5}`, `mt = {24×5, 25×12}`), so the estimate is correct for any
size / colour-count / framed configuration the playground offers, and hints are
pre-placed exactly as the reference does.

## Caveat

Complex theory is a **first-moment** estimate — it treats the edge colours as
independently drawn. McGavin's small-puzzle calibration puts it within a factor
~2 of true counts. It does **not** capture the global-distinctness wall
(see the site's *Entropy and the area law* page); use it for tree shape and
scan-order choice, not as a true count or a bound.
