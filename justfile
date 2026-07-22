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

# Reproduce the forbidden-patterns topic's results.
research-forbidden-patterns:
    cd research/topics/forbidden-patterns/compute && cargo run --release > ../results/feasibility.json

# Reproduce the subgrid-placement-counts topic's results.
research-subgrid:
    cd research/topics/subgrid-placement-counts/compute && cargo run --release > ../results/reference-table.json

# Reproduce the phase-transition topic's results.
research-phase-transition:
    cd research/topics/phase-transition/compute && cargo run --release > ../results/color-split.json

# Reproduce the no-forced-moves topic's results.
research-no-forced-moves:
    cd research/topics/no-forced-moves/compute && cargo run --release > ../results/partner-counts.json

# Reproduce the piece-theft topic's results.
research-piece-theft:
    cd research/topics/piece-theft/compute && cargo run --release > ../results/piece-theft.json

# Reproduce the prune-vs-speed topic's results.
research-prune-vs-speed:
    cd research/topics/prune-vs-speed/compute && cargo run --release > ../results/prune-vs-speed.json

# Reproduce the rare-color-geography topic's results.
research-rare-color-geography:
    cd research/topics/rare-color-geography/compute && cargo run --release > ../results/color-geography.json

# Reproduce the entropy-area-law topic's results.
research-entropy-area-law:
    cd research/topics/entropy-area-law/compute && cargo run --release > ../results/entropy.json

# Recompute every bundled record board's score from its edges (verification).
research-record-boards:
    cd research/topics/record-boards/compute && cargo run --release > ../results/verified-scores.json

# Reproduce the clue-puzzle-pieces topic's results (SAT-solve + verify the four
# recovered clue puzzles). Requires kissat and python3; see the topic's run.sh.
research-clue-puzzle-pieces:
    cd research/topics/clue-puzzle-pieces && ./run.sh

# Reproduce the rigidity-sat-halo topic's results (self-test + SAT halo search).
research-rigidity-sat-halo:
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
