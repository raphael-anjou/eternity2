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

# --- Research --------------------------------------------------------------

# Validate every research topic and regenerate research/index.json.
research-index:
    node research/build-index.mjs

# Reproduce the forbidden-patterns topic's results.
research-forbidden-patterns:
    cd research/topics/forbidden-patterns/compute && cargo run --release > ../results/feasibility.json

# Reproduce the subgrid-placement-counts topic's results.
research-subgrid:
    cd research/topics/subgrid-placement-counts/compute && cargo run --release > ../results/reference-table.json
