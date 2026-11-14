# Amendments to PREREG.md

Each entry: what, why, when. Written before any test-seed tree was replayed.

1. **Exhaustive soundness check widened** (2026-11-13, 00:38). What: gate 3
   runs on 5x5 c=4 and c=5 seeds 1..20 as preregistered, plus 5x5 c=6 and
   6x6 c=5 generated boards with seeds 5001..5020 (not ladder seeds; the
   ladder test seeds are 1..30 and calibration 1001..1010). Why: the
   preregistered boards gave only 11 to 22 kill points per arm, too few to
   exercise the check; the added boards give about 1,400 to 2,000 per arm.
   Nothing is removed. Colours are the ladder generator's interior colour
   count (`hard_ladder::instance::generate(n, c, seed)`, which adds the frame
   colours).
