# Common brief for the field surveys of 2026-11-13

Purpose: the program has priced every route it examined to a full solution of Eternity II and found none within 17 orders of magnitude. Before inventing anything else, map what other fields already know about problems with this structure. You survey ONE field. You do not invent a method and you do not run experiments.

## The problem, in the terms another field would use
- 256 distinct square tiles, 4 edge colours each, to be placed with rotation on a 16x16 grid so that all 480 internal edges match and the outside is grey. 22 colours: 5 appear only on the frame ring, 17 only in the interior. Five tiles are fixed (clues). NP-complete in general (Demaine and Demaine 2007).
- The instance is at the hardest density: the number of colours was chosen so that the expected number of solutions is small (about 14,702 with one clue, about 1 with five, first-moment estimates). One solution is known to exist by construction (planted) and is unpublished. Nobody has found any solution since 2007. Best partial: 470 of 480 edges.
- Measured cost of the best known complete search (row-by-row backtracking with the five clues): 10^40.46 nodes = 2^134.4, on the real tiles; a first-moment model of the search tree predicts it to 0.03 digits. So the tree is essentially that of a random instance with the same colour counts.
- Where the cost sits: about 30 of 39 digits at the peak are the multiplicity of partial frames (the border). Given the true border, the interior is cheap. Local reasoning (arc consistency, counting, corner tables) sees nothing beyond the corners.
- What kills counting and algebraic methods so far: each tile is used exactly once (an all-different constraint over 256 tiles), so any transfer-matrix or dynamic-programming state must carry the set of used tiles.
- Files to read for detail, all under <repo>/research/program/night/: `2026-11-12/REPORT.md`, `2026-11-13/ROUTES.md` (every route already examined, with its verdict; you MUST check each method you report against it and cite the row, or say none exists), `2026-11-12/memos/` (six "solve path" memos and their reviews), `2026-11-12/memos/literature-2026.md` with its review (a first literature pass; do not repeat it, extend it).

## What to produce
One file: `<repo>/research/program/night/2026-11-13/surveys/<your-slug>.md`. Write only that file. Do not commit. At most one CPU core, no builds, no runs beyond a tiny stdlib calculation. No em dashes. Never put this project's or its owner's name in a web request. 90 minutes.

Structure of the file:
1. **The analog.** Which problem(s) in your field have the same structure, and in what precise sense (what maps to tiles, to the all-different constraint, to the planted solution, to the density). Say where the analogy breaks.
2. **What the field knows**, as a table: method or theorem | what it achieves on the analog (exponent, threshold, guarantee) | the source, opened today, with URL, authors, year, and the sentence or theorem number you rely on | grade (theorem, experiment, heuristic claim) | already examined here? (ROUTES.md row, or "no row").
3. **Hardness evidence.** Does the field have results saying that instances like this are hard for whole classes of algorithms (lower bounds, barriers, phase-transition arguments)? State them exactly, with their assumptions, and whether those assumptions hold for this instance (planted, fixed size, 22 colours, grid). A barrier result is as valuable as a method.
4. **Transfer candidates**, at most three, each as a testable statement: what the method would be on Eternity II, its cost in bits by an explicit back-of-envelope formula against the 134.4-bit baseline (show the arithmetic; state which factor is a guess), the cheapest experiment on a small board or on the real tiles that could kill it, and your odds. If nothing transfers, say so and say why; do not pad.
5. **What you could not open or verify**, and the three most relevant papers to read in full.
Rules of evidence: every claim carries its source and a grade; "I recall" is not a source; open the paper (arXiv, HAL, DBLP, publisher page) and quote the result; a claim you could not open is listed in section 5, not in the table. Prefer results after 2015 but do not skip the classical ones. If a search engine summary and the paper disagree, the paper wins.
Final message to the lead: the single most important thing your field says about this problem, the best transfer candidate with its bits and odds, and the strongest barrier result, in under 12 lines.
