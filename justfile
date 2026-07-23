# Eternity II community site: common tasks.
#
# Install `just` (https://just.systems) then run `just` to list tasks, or
# `just <task>`. Everything here is a thin wrapper over the underlying cargo /
# pnpm / node commands, so you can always run those directly too.

# Show the task list (default).
default:
    @just --list

# --- First run -------------------------------------------------------------

# One-shot setup: build the engine to WASM and install web deps.
setup: wasm
    cd web && pnpm install

# Run the site locally with hot reload at http://localhost:5173
dev:
    cd web && pnpm dev

# --- Engine ----------------------------------------------------------------

# Run the engine's Rust tests (includes the bucas interop cross-checks).
test:
    cd engine && cargo test --release

# Compile the engine to WebAssembly, straight into the web app.
wasm:
    cd engine && wasm-pack build --target web --out-dir ../web/src/engine/pkg --release

# Native single-core throughput benchmark.
bench:
    cd engine && cargo run --release --bin bench

# Regenerate the difficulty data the Algorithms page charts.
stats:
    cd engine && cargo run --release --bin stats > ../web/src/data/difficulty.json

# --- Web -------------------------------------------------------------------

# Type-check the web app.
typecheck:
    cd web && pnpm typecheck

# Lint the web app.
lint:
    cd web && pnpm lint

# Production build (also type-checks first).
build:
    cd web && pnpm build

# Everything CI checks, in order: engine tests, typecheck, lint, build.
check: test typecheck lint build

# --- Starter kit -----------------------------------------------------------

# Build the starter kit and run its tests (official-set + pinned-hint + outcome guards).
starter-kit:
    cd research/starter-kit && cargo build --release && cargo test --release

# Run one of the kit's examples end to end, e.g.
# `just starter-kit-example generate` or `just starter-kit-example sweep`.
starter-kit-example name="score":
    cd research/starter-kit && cargo run --release --example {{name}}

# --- Research --------------------------------------------------------------

# Shared research experiments. `just experiments` lists them;
# `just experiments single-core-benchmark` runs one (see that dir's justfile).
mod experiments 'research/experiments/justfile'

# Validate every research topic and regenerate research/index.json.
research-index:
    node research/build-index.mjs

# Top up the local groups.io archive to the current tail via the REST API.
# Idempotent; needs GROUPS_IO_SCRAPING_KEY from the parent .env. The archive
# itself lives outside the repo and is never committed (see GROUPSIO_API.md).
archive-refresh:
    set -a; . ../.env; set +a; python3 research/community/refresh_archive.py

# Reproduce the forbidden-patterns topic's results.
research-forbidden-patterns:
    cd research/topics/forbidden-patterns/compute && cargo run --release > ../results/feasibility.json

# Reproduce the subgrid-placement-counts topic's results.
research-subgrid:
    cd research/topics/subgrid-placement-counts/compute && cargo run --release > ../results/reference-table.json

# Reproduce the phase-transition topic's results.
research-phase-transition:
    cd research/topics/phase-transition/compute && cargo run --release > ../results/color-split.json

# Reproduce the hardness-peak sweep (solver effort vs interior-color count).
# Runs BOTH regimes in order: the 8x8 traversable-peak sweep, then the 16x16
# real-board sweep, producing results/hardness-peak-8x8.json and
# results/hardness-peak-16x16.json. ~45 min on 8 cores; resumable (reruns skip
# completed (c, seed) cells). For a single regime, call run.sh with E2_SIZE set.
research-hardness-peak:
    cd research/topics/hardness-peak && ./run-both.sh

# Reproduce the no-forced-moves topic's results.
research-no-forced-moves:
    cd research/topics/no-forced-moves/compute && cargo run --release > ../results/partner-counts.json

# Reproduce the piece-theft topic's results.
research-piece-theft:
    cd research/topics/piece-theft/compute && cargo run --release > ../results/piece-theft.json

# Reproduce the prune-vs-speed topic's results.
research-prune-vs-speed:
    cd research/topics/prune-vs-speed/compute && cargo run --release > ../results/prune-vs-speed.json

# Reproduce the irreducible-hard-region localization topic. Runs the 40-board
# row-major vs random-order sweep, then the 8-board budget-insensitivity check.
# ~4 min single core.
research-irreducible-hard-region:
    cd research/topics/irreducible-hard-region/compute && cargo run --release -- --n 16 --seed-lo 1 --seed-hi 40 --node-cap 1500000 --restart-cap 150000 > ../results/hard_region.json
    cd research/topics/irreducible-hard-region/compute && cargo run --release -- --n 16 --seed-lo 1 --seed-hi 8 --node-cap 4000000 --restart-cap 400000 > ../results/hard_region_budget4m.json

# Reproduce the rare-color-geography topic's results.
research-rare-color-geography:
    cd research/topics/rare-color-geography/compute && cargo run --release > ../results/color-geography.json

# Reproduce the entropy-area-law topic's results.
research-entropy-area-law:
    cd research/topics/entropy-area-law/compute && cargo run --release > ../results/entropy.json

# Recompute every bundled record board's score from its edges (verification).
research-record-boards:
    cd research/topics/record-boards/compute && cargo run --release > ../results/verified-scores.json

# Reproduce the border-mismatch-share topic's results (split each high board's
# unmatched edges into interior-interior / interior-border / border-border).
research-border-mismatch-share:
    cd research/topics/border-mismatch-share/compute && cargo run --release > ../results/mismatch-split.json

# Reproduce the clue-puzzle-pieces topic's results (SAT-solve + verify the four
# recovered clue puzzles). Requires kissat and python3; see the topic's run.sh.
research-clue-puzzle-pieces:
    cd research/topics/clue-puzzle-pieces && ./run.sh

# Reproduce the rigidity-sat-halo topic's results (self-test + SAT halo search).
research-rigidity-sat-halo:
    cd research/topics/rigidity-sat-halo/compute && cargo run --release -- --selftest
    cd research/topics/rigidity-sat-halo && ./run.sh

# Regenerate the experiments-log data from the research vault concepts.
research-experiments-log:
    cd research/topics/experiments-log && python3 extract.py > ../../../web/src/data/experiments.json

# Validate every internal link in the research wiki content (MDX + frontmatter).
research-wiki-check:
    node web/scripts/check-research-links.mjs

# Audit research citations: msg numbers vs the local archive, external URLs, missing sources.
research-citations-check:
    node web/scripts/check-research-citations.mjs

# Audit research prose style: em dashes and recurring LLM tics (quotes exempt).
research-style-check:
    node web/scripts/check-research-style.mjs

# Reproduce the cas-annular topic's results (20 CAS frames + 8 baseline frames).
research-cas-annular:
    cd research/topics/cas-annular/compute && cargo run --release -- 20 8 1024 45 > ../results/cas_plateau.json

# Reproduce the frame-manifold topic's results (500 frames + the seed-7001 robustness pass).
research-frame-manifold:
    cd research/topics/frame-manifold/compute && cargo run --release > ../results/manifold.json
    cd research/topics/frame-manifold/compute && cargo run --release -- --frames 100 --walk 100 --seed0 7001 > ../results/manifold_seed7001.json

# Reproduce the frame-is-not-the-basin topic's results (16 distinct strong frames
# each completed by one fixed interior producer + the seed-7001 robustness pass).
research-frame-is-not-the-basin:
    cd research/topics/frame-is-not-the-basin/compute && cargo run --release -- --frames 16 --beam 128 --seeds 3 --seed0 1 > ../results/frame_first.json
    cd research/topics/frame-is-not-the-basin/compute && cargo run --release -- --frames 12 --beam 128 --seeds 2 --seed0 7001 > ../results/frame_first_seed7001.json

# Reproduce the ledger-prune topic's results (soundness gate + A/B sweep + the 464 certificate rows).
research-ledger-prune:
    cd research/topics/ledger-prune/compute && cargo run --release -- --mode cert --cap-secs 300 > ../results/cert_phase2.json

# Reproduce the beam-width-smc topic's results (width law + tie-break + SMC paired arms).
research-beam-width-smc:
    cd research/topics/beam-width-smc/compute && cargo run --release > ../results/beam-width-smc.json

# Reproduce the tail-finishability-frostline topic's results (120 tops, exact tail labels, BP factors).
research-frostline:
    cd research/topics/tail-finishability-frostline/compute && uv run frostline.py --seeds 120 --betas 1,2,3,4,5,6 --out ../results/frostline_r15.json

# Reproduce the scaling-ladder topic's results (4 rungs x 8 seeds x 2 baseline solvers).
research-scaling-ladder:
    cd research/topics/scaling-ladder/compute && cargo run --release > ../results/summary.json

# Reproduce the sigma-cycles topic's results (all ordered same-piece-set board
# pairs: cycle structure + every-proper-prefix partial-application score curves).
research-sigma-cycles:
    cd research/topics/sigma-cycles/compute && cargo run --release > ../results/sigma-cycles.json

# Reproduce the no-height-function topic's results (odd dual cores + non-conserved
# per-color currents on committed high-scoring boards; qualitative obstruction).
research-no-height-function:
    cd research/topics/no-height-function/compute && cargo run --release > ../results/no_height_function.json

# Reproduce the permutation-code-wall topic's results (WEFT coding-theory lens:
# 480 checks + permutation code + border code structural facts, and the
# score = 480 - breaks syndrome identity on codewords with known break counts).
research-permutation-code-wall:
    cd research/topics/permutation-code-wall/compute && cargo run --release > ../results/permutation_code_wall.json

# Reproduce the exact-tail-endgame topic's results (frozen top, CP-SAT exact
# optimisation of the bottom band vs the producer's own tail; delta distribution
# across producer seeds, incoming-break cap 1 vs 2).
research-exact-tail-endgame:
    cd research/topics/exact-tail-endgame/compute && uv run tailforge.py --seeds 24 --rows 1,2 --caps 1,2 --band-seeds 1:24,2:4 --time-limit 45 --workers 8 --out ../results/tailforge.json

# Reproduce the flux-invariants topic's results (official-instance facts, the
# flux-law complex/real/F2 ranks + census orthogonality, and the mod-2 endgame
# certificate catch curve on planted 8x8 boards).
research-flux-invariants:
    cd research/topics/flux-invariants/compute && cargo run --release > ../results/flux_invariants.json
