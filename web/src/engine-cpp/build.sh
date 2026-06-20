#!/usr/bin/env bash
# Build the Eternity II C++ engine to freestanding wasm32 with clang++ (NOT
# emscripten). Produces engine.wasm next to engine.cpp.
#
# Toolchain notes (macOS):
#   - Apple's /usr/bin/clang++ has NO wasm32 codegen target, so we compile the
#     wasm object with Homebrew LLVM clang (`brew install llvm`), which does.
#   - Homebrew LLVM does not bundle wasm-ld unless `lld` is also installed, so we
#     link with whatever wasm linker we can find: a standalone wasm-ld, or the
#     `rust-lld` that ships with any rustup toolchain (invoked in wasm flavor).
# Everything is freestanding: -nostdlib, no libc, no exceptions, no rtti.
set -euo pipefail
cd "$(dirname "$0")"

# --- locate a clang++ that can target wasm32 --------------------------------
CXX=""
for cand in \
  "${CLANGXX:-}" \
  /opt/homebrew/opt/llvm/bin/clang++ \
  /usr/local/opt/llvm/bin/clang++ \
  "$(command -v clang++ || true)"; do
  [ -n "$cand" ] || continue
  if "$cand" --print-targets 2>/dev/null | grep -qi wasm32; then CXX="$cand"; break; fi
done
if [ -z "$CXX" ]; then
  echo "error: no clang++ with a wasm32 target found (install: brew install llvm)" >&2
  exit 1
fi
echo "clang++:  $CXX"

# --- locate a wasm linker ---------------------------------------------------
WASMLD=""
if command -v wasm-ld >/dev/null 2>&1; then
  WASMLD="$(command -v wasm-ld)"
else
  # rustup ships rust-lld; `rust-lld -flavor wasm` is a wasm-ld.
  for rl in "$HOME"/.rustup/toolchains/*/lib/rustlib/*/bin/rust-lld; do
    [ -x "$rl" ] && { WASMLD="$rl -flavor wasm"; break; }
  done
fi
if [ -z "$WASMLD" ]; then
  echo "error: no wasm linker found (need wasm-ld or a rustup toolchain)" >&2
  exit 1
fi
echo "wasm-ld:  $WASMLD"

CXXFLAGS=(
  --target=wasm32
  -nostdlib -ffreestanding -fno-exceptions -fno-rtti
  -O3 -std=c++20 -Wall -Wextra
)

# 1) compile to a relocatable wasm object
"$CXX" "${CXXFLAGS[@]}" -c engine.cpp -o engine.o

# 2) link to a standalone module: no entry point, export everything, allow the
#    (unused) undefined externs, give it room for the static solver pool.
$WASMLD \
  --no-entry \
  --export-dynamic \
  --allow-undefined \
  --initial-memory=2097152 \
  -o engine.wasm \
  engine.o

rm -f engine.o

# 3) optional binaryen size pass
if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt -O3 engine.wasm -o engine.wasm
  echo "wasm-opt applied"
fi

ls -l engine.wasm
