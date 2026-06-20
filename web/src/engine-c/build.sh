#!/usr/bin/env bash
# Build the C Eternity II engine to freestanding wasm32 with clang (NO
# emscripten). Also (re)generates official_data.h from the canonical CSV, and
# optionally shrinks the wasm with wasm-opt if it is on PATH.
#
# Usage: ./build.sh
# Output: engine.wasm (committed so the site builds without a C toolchain).
set -euo pipefail
cd "$(dirname "$0")"

CSV="../../../engine/data/official_eternity2.csv"

# Apple's stock clang lacks the wasm32 backend. Pick a wasm-capable clang:
# honour $CLANG, else try Homebrew LLVM, else fall back to plain `clang`.
pick_clang() {
  if [ -n "${CLANG:-}" ]; then echo "$CLANG"; return; fi
  for c in /opt/homebrew/opt/llvm/bin/clang /usr/local/opt/llvm/bin/clang clang; do
    if "$c" -print-targets 2>/dev/null | grep -qi wasm32; then echo "$c"; return; fi
  done
  echo clang   # last resort; will error loudly if it cannot target wasm32
}
CLANG="$(pick_clang)"
echo "using clang: $CLANG"

# clang invokes `wasm-ld` for the wasm32 link step; it must be on PATH. The
# Homebrew `lld` formula ships it. Add common locations so the link succeeds
# even when lld's bin dir isn't already exported.
for d in /opt/homebrew/opt/lld/bin /usr/local/opt/lld/bin "$(dirname "$CLANG")"; do
  if [ -x "$d/wasm-ld" ]; then PATH="$d:$PATH"; fi
done
export PATH
if ! command -v wasm-ld >/dev/null 2>&1; then
  echo "error: wasm-ld not found on PATH. Install it with: brew install lld" >&2
  exit 1
fi
echo "using wasm-ld: $(command -v wasm-ld)"

# --- 1. Regenerate the embedded official set from the canonical CSV. --------
# Decodes the 16-bit binary color words (65535 -> 0 border) to decimal and
# emits hint records {pos, piece, rot}. Keeps engine.c source-reproducible.
if [ -f "$CSV" ]; then
  awk -F, '
  BEGIN { nh=0 }
  function b2d(s,   i,v){ v=0; for(i=1;i<=length(s);i++) v=v*2+substr(s,i,1); return v }
  function col(s,   v){ v=b2d(s); return (v==65535)?0:v }
  NR==1 { size=$1; next }
  {
    id=NR-2;
    pieces[id]=sprintf("%d,%d,%d,%d", col($1),col($2),col($3),col($4));
    npieces=id+1;
    x=$5+0; y=$6+0; rot=$7+0;
    if (x!=0||y!=0||rot!=0){ hints[nh]=sprintf("{%d,%d,%d}", y*size+x, id, rot); nh++ }
  }
  END {
    printf("// Auto-generated from engine/data/official_eternity2.csv (DO NOT EDIT BY HAND).\n");
    printf("// Regenerate via build.sh. Edges URDL, color words decoded (65535->0 border).\n");
    printf("#define E2_OFFICIAL_SIZE %d\n", size);
    printf("#define E2_OFFICIAL_PIECES %d\n", npieces);
    printf("#define E2_OFFICIAL_HINTS %d\n", nh);
    printf("static const unsigned char E2_OFFICIAL_EDGES[%d][4] = {\n", npieces);
    for (i=0;i<npieces;i++) printf("  {%s},\n", pieces[i]);
    printf("};\n");
    printf("// Hint = {pos, piece, rot}\n");
    printf("static const short E2_OFFICIAL_HINT[%d][3] = {\n", nh);
    for (i=0;i<nh;i++) printf("  %s,\n", hints[i]);
    printf("};\n");
  }' "$CSV" > official_data.h
  echo "regenerated official_data.h from $CSV"
else
  echo "warning: $CSV not found; using committed official_data.h"
fi

# --- 2. Compile to freestanding wasm32. ------------------------------------
# -nostdlib: no libc. --no-entry: it's a library, not an executable.
# --export-dynamic plus the per-function export_name attributes expose the ABI.
# Static fixed-size buffers (sized for 16x16) mean no allocator is needed.
"$CLANG" \
  --target=wasm32 \
  -nostdlib \
  -ffreestanding \
  -O3 \
  -Wl,--no-entry \
  -Wl,--export-dynamic \
  -Wl,--allow-undefined \
  -Wl,--initial-memory=2097152 \
  -o engine.wasm \
  engine.c

echo "built engine.wasm ($(wc -c < engine.wasm) bytes)"

# --- 3. Optional size optimization. ----------------------------------------
if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt -O3 engine.wasm -o engine.wasm
  echo "wasm-opt -O3 applied ($(wc -c < engine.wasm) bytes)"
fi

# --- 4. Native build for ABI-independent golden diffing (debug aid). --------
# (Not required by the site; produced only if you want to diff against golden.)
if [ "${1:-}" = "--native" ]; then
  clang -O2 engine.c -o engine_native
  echo "built engine_native"
fi
