#!/usr/bin/env bash
# Run every side-quest engine and show that they all solve the same puzzles the
# Rust engine does.
#
#   3x3  -> Lua, COBOL, and Brainfuck (the Brainfuck solver is only fast enough
#           for 3x3).
#   4x4, 5x5 -> Lua and COBOL only (Brainfuck would take far too long).
#
# Each engine is checked against the board/stats the Rust engine produced. This
# is a convenience wrapper; each engine also has its own test script
# (engine-lua/spec.lua, engine-cobol/test.sh, engine-brainfuck/test_solver.sh).
#
# Usage:  ./run-all.sh
set -uo pipefail
cd "$(dirname "$0")"
ROOT="$(pwd)"

bold() { printf '\033[1m%s\033[0m\n' "$*"; }
rule() { printf '%s\n' "------------------------------------------------------------"; }

# --- locate interpreters -----------------------------------------------------
LUA="$(command -v lua || command -v lua5.4 || command -v lua5.3 || true)"
COBC="$(command -v cobc || true)"
RUSTC="$(command -v rustc || true)"
NODE="$(command -v node || true)"
PY="$(command -v python3 || true)"

have_lua=1;   [ -z "$LUA" ]  && have_lua=0
have_cobol=1; [ -z "$COBC" ] && have_cobol=0
have_node=1;  [ -z "$NODE" ] && have_node=0
have_py=1;    [ -z "$PY" ]   && have_py=0

# ============================================================================
bold "Eternity II — running every side-quest engine"
echo "Node (TS/C/C++ parity): ${NODE:-(not found — skipping)}"
echo "Python:    ${PY:-(not found — skipping)}"
echo "Lua:       ${LUA:-(not found — skipping)}"
echo "COBOL:     ${COBC:-(not found — skipping)}"
echo "Brainfuck: Rust interpreter ${RUSTC:+(rustc found)}${RUSTC:-(rustc not found, using bf.py)}"
echo

# ============================================================================
# ISO PORTS — full golden parity (RNG + 9 paths + solver counts + official set).
# These are the serious, functionality-matching ports. TS/C/C++ are also the
# browser-switchable backends (web's VITE_ENGINE switch).
# ============================================================================
if [ "$have_node" = 1 ]; then
  rule; bold "TYPESCRIPT — pure-TS port, full golden parity"
  ( "$NODE" "$ROOT/../web/src/engine-ts/parity.mjs" | tail -1 )
  echo
  rule; bold "C — clang→WASM port, full golden parity (wasm under node)"
  ( "$NODE" "$ROOT/../web/src/engine-c/parity.mjs" | tail -1 ) \
    || echo "  (engine.wasm missing? rebuild: web/src/engine-c/build.sh)"
  echo
  rule; bold "C++ — clang++→WASM port, full golden parity (wasm under node)"
  ( "$NODE" "$ROOT/../web/src/engine-cpp/parity.mjs" | tail -1 ) \
    || echo "  (engine.wasm missing? rebuild: web/src/engine-cpp/build.sh)"
  echo
else
  rule; echo "TypeScript/C/C++ parity skipped (no node)."; echo
fi

if [ "$have_py" = 1 ]; then
  rule; bold "PYTHON — pure-Python port, full golden parity"
  ( cd engine-python && "$PY" spec.py | tail -1 )
  echo
  for args in "3 2 3" "4 11 5" "5 3 6"; do
    set -- $args
    bold "PYTHON — solve a generated ${1}x${1} (seed ${2}, ${3} colors)"
    ( cd engine-python && "$PY" demo.py "$1" "$2" "$3" | sed -n '3,5p' )
    echo
  done
else
  rule; echo "Python skipped (no python3)."; echo
fi

# ============================================================================
# LUA — its spec already cross-checks generated puzzles + the official set
# against Rust golden data (3x3 through 16x16), so running it covers all sizes.
# ============================================================================
if [ "$have_lua" = 1 ]; then
  rule; bold "LUA — full parity suite (generated + official, all sizes)"
  ( cd engine-lua && "$LUA" spec.lua | tail -1 )
  echo
  # size seed colors — few colors (5/6) so the search actually backtracks
  # (max colors makes every placement forced and the solve trivial).
  for args in "3 2 3" "4 11 5" "5 3 6"; do
    set -- $args
    bold "LUA — solve a generated ${1}x${1} (seed ${2}, ${3} colors)"
    ( cd engine-lua && "$LUA" demo.lua "$1" "$2" "$3" | sed -n '3,6p' )
    echo
  done
else
  rule; echo "LUA skipped (no interpreter)."; echo
fi

# ============================================================================
# COBOL — compile once, then solve the 3x3 / 4x4 / 5x5 data files. Each prints
# a RESULT line that matches the Rust engine field-for-field.
# ============================================================================
if [ "$have_cobol" = 1 ]; then
  rule; bold "COBOL — compile + solve 3x3, 4x4, 5x5"
  ( cd engine-cobol \
    && cobc -x -free eternity2.cbl -o eternity2 \
    && for f in data/p_3_3_1.txt data/p_4_4_11.txt data/p_5_5_3.txt; do
         ./eternity2 "$f"
       done )
  echo
  bold "COBOL — parity test (vs Rust RESULT lines)"
  ( cd engine-cobol && ./test.sh | tail -1 )
  echo
else
  rule; echo "COBOL skipped (no cobc)."; echo
fi

# ============================================================================
# BRAINFUCK — 3x3 only (4x4 ~100s, 5x5 far longer). Compiles the solver and
# checks every 3x3 board against the Rust engine's.
# ============================================================================
rule; bold "BRAINFUCK — 3x3 solver parity (vs Rust boards)"
( cd engine-brainfuck && ./test_solver.sh | tail -5 )
echo

rule; bold "Done."
