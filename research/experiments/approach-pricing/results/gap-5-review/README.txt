Second analysis scripts for notes/gap-5-tile-set-structure.md (2026-11-13). One core, nice 10, no run
over 15 minutes. Official pieces read from
research/experiments/single-core-benchmark/engine/data/puzzles/size_16_official_eternity.csv

rcommon.py                 loader (own), rotations, exact integer Smith form, rank mod p
rfill.py                   filling of the empty cells, tiles reusable or each once, pinned cells
r1_lattice.py              the abelian invariant group from the definition; three positive controls
r1b_cover_invariant.py     the k x k covers; the extra invariant of the covers, named and verified
r2b_depth112_flags.py      dead / arc consistency / cell-level LP per prefix, depths 112 to 144
r2a_prefix_death.py        the memo's 184 prefixes regenerated; proven-dead share; arc consistency;
                           next-row test; Knuth estimate of level sizes. Writes prefixes_noclue.json
r3_structure.py            colour counts, seam, rim digraph, colour-pair graphs, checkerboard parity, clues
r3b_automorphisms.py       colour automorphisms by exhaustive backtracking, with two controls
r4_tile_count_invariants.py  tile invariants in Pak's sense: certificate per prefix
r5_clue_prefixes.py        prefixes with the five clues pinned, and every test on them
r5b_clue_more.py           locality of the clue refutations, 48 more restarts, certificate on deep clue paths
                           (rfill.py's cell order around pins changed after r5 ran; see the review)
r6_integer_and_power.py    the memo's test 1 rebuilt: mod 2, mod 3 and 5, 0/1 program, cell-level LP
r7_nonabelian_quotients.py all homomorphisms of the tile group into small non-abelian groups
r8_count_groups.py         completeness of the flux family among count invariants, four resolutions
run_r6_r7.sh               batch for r6 and r7

Commands:
  python3 -I r1_lattice.py | r2a_prefix_death.py 400000 | r3_structure.py | r3b_automorphisms.py
  uv run --with python-flint python r1b_cover_invariant.py | r4_... 6 400 A 0 | r5_... 16 1 400 |
      r7_... 8 | r8_count_groups.py 16 0 1.35
  uv run --with numpy --with scipy --with python-flint python r6_integer_and_power.py MODE ...
Each .out file is the output of the script of the same name.
