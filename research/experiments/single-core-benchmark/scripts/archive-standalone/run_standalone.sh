#!/usr/bin/env bash
# run_standalone.sh — uniform single-core, ~60s-wall wrappers around the 4
# "standalone" solver binaries (benchmark grid). Each function:
#
# run_producer <variant.csv> <seed> <budget_s> <out.url>
# run_blackwood <variant.csv> <seed> <budget_s> <out.url>
# run_verhaard <variant.csv> <seed> <budget_s> <out.url>
# run_alns <variant.csv> <seed> <budget_s> <out.url>
#
# runs the engine SINGLE-CORE for approximately budget_s seconds wall-clock,
# writes a bucas .url to out.url, and ALWAYS prints:
# - a line containing `canonical_score=NNN/480` from an independent
# target/release/verify_bucas re-score (never trust the binary's
# self-reported score);
# - a final line `nodes=<N> nps=<N> nps_unit=<unit>` in the engine's
# NATIVE unit of work (producer=beam-nodes/s, blackwood/verhaard=
# search-nodes/s straight from the binary's own throughput log,
# alns=iters/s) so the grid runner can parse search-speed alongside
# score.
# This file is meant to be `source`d, not executed directly (functions only;
# no `set -e` so a failing engine still falls through to the mandatory
# url+score step).
#
# Calibration data, exact repro commands, and measured wall-times/scores are
# in CALIBRATION.md in this same directory. Re-run that calibration if the
# machine or binaries change.
#
# Single-core notes (see CALIBRATION.md for the full audit):
# - producer_trie: no threading in the binary at all (verified by
# grep) -- inherently single-core.
# - blackwood_bt / verhaard_faithful_v2: spawn exactly ONE worker thread
# (std::thread::Builder with a 256MB stack, for stack-depth headroom on
# the 256-deep recursive DFS) and `.join()` it synchronously before doing
# anything else -- single-core in practice (one thread runs at a time).
# - alns_only: `run_alns` (single-chain entry point used here) has no
# rayon calls on the SA repair path (the default, `--repair-kind sa`).
# rayon only appears in `run_alns_portfolio` / `run_alns_pt` (unused
# here) and in the CP-repair path (`RepairKind::Cp`, avoided here; the
# default repair kind is `sa`). RAYON_NUM_THREADS=1 is exported anyway as
# a defensive belt-and-braces pin in case a future default changes.
#
# All temp working dirs are timestamped + include the PID so concurrent runs
# (e.g. via run_grid.py's ThreadPoolExecutor) never collide or overwrite.

set -uo pipefail

# Force a C locale for numeric formatting: on a non-English locale (e.g.
# fr_FR), `awk`'s printf uses a comma decimal separator ("58,48" instead of
# "58.48"), which breaks every downstream parser (grid runner, this script's
# own `[[ ... ]]` numeric tests). All arithmetic in this file assumes '.'.
export LC_ALL=C
export LC_NUMERIC=C

# Resolve the repo root (v2/) from this file's location, so the functions
# work regardless of the caller's cwd.
_RSA_HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
V2_ROOT="$(cd "${_RSA_HERE}/../../.." && pwd)"

PRODUCER_BIN="${V2_ROOT}/target/release/producer_trie"
BLACKWOOD_BIN="${V2_ROOT}/target/release/blackwood_bt"
VERHAARD_BIN="${V2_ROOT}/target/release/verhaard_faithful_v2"
ALNS_BIN="${V2_ROOT}/target/release/alns_only"
DUMP_BOARD_JSON_BIN="${V2_ROOT}/target/release/dump_board_json"
VERIFY_BIN="${V2_ROOT}/target/release/verify_bucas"

export RAYON_NUM_THREADS=1

# _rsa_scratch <tag> -- new timestamped+PID scratch dir under
# output/bench-grid-scratch/, never overwrites a previous run's dir.
_rsa_scratch() {
 local tag="$1"
 local d="${V2_ROOT}/output/bench-grid-scratch/${tag}_$(date +%Y%m%dT%H%M%S)_$$"
 mkdir -p "$d"
 echo "$d"
}

# _rsa_verify_and_emit <variant.csv> <url-string> <out.url>
# Writes url-string to out.url (even if empty/invalid -- hard rule: always
# emit a url artifact), then independently re-scores it with verify_bucas
# and prints the resulting `canonical_score=NNN/480` line. If url-string is
# empty (engine produced nothing at all, e.g. crashed before any board),
# writes a sentinel file and prints canonical_score=0/480 without invoking
# verify_bucas (which requires a decodable board_edges parameter).
_rsa_verify_and_emit() {
 local variant_csv="$1"
 local url="$2"
 local out_url="$3"
 mkdir -p "$(dirname "$out_url")"
 if [[ -z "$url" ]]; then
 echo "NO_BOARD_PRODUCED" > "$out_url"
 echo "canonical_score=0/480 (NO BOARD PRODUCED -- see stderr above)"
 return 1
 fi
 printf '%s' "$url" > "$out_url"
 "$VERIFY_BIN" "$variant_csv" "$url"
 return 0
}

# ---------------------------------------------------------------------------
# 1. PRODUCER (producer_trie) -- wide-beam, order comb:14.
# Not natively time-bounded (runs a fixed beam to completion); calibrated
# beam=80000 completes in ~55s single-core on this machine (see
# CALIBRATION.md). budget_s is accepted for interface uniformity but the
# beam is fixed at calibration time, not derived from budget_s, because
# the beam<->wall-time relationship is nonlinear and was hand-calibrated
# for the ~60s target. A `timeout` guard at budget_s+15s is still applied
# so a pathological variant can never hang the grid.
# ---------------------------------------------------------------------------
run_producer() {
 local variant_csv="$1" seed="$2" budget_s="$3" out_url="$4"
 local beam=80000
 local scratch; scratch="$(_rsa_scratch "producer")"
 local emit="${scratch}/best.url"

 local t0 t1 wall
 t0=$(date +%s.%N)
 timeout "$(( budget_s + 15 ))" "$PRODUCER_BIN" "$variant_csv" \
 --order comb:14 --beam "$beam" --tol 0 --seed "$seed" \
 --emit-best "$emit" \
 > "${scratch}/stdout.log" 2> "${scratch}/stderr.log"
 local rc=$?
 t1=$(date +%s.%N)
 wall=$(awk -v a="$t0" -v b="$t1" 'BEGIN{printf "%.2f", b-a}')
 if [[ $rc -ne 0 ]]; then
 echo "run_producer: engine exited rc=$rc (see ${scratch}/stderr.log)" >&2
 tail -5 "${scratch}/stderr.log" >&2 2>/dev/null || true
 fi

 # Node accounting: the beam producer has no per-node counter, but its
 # search is EXACTLY `layers` sequential frontier-expansion steps, each
 # expanding up to `beam_width` surviving parents -- so
 # layers*beam_width is the (tight, since beam saturates almost
 # immediately) count of beam-node expansions performed. `layers` is
 # printed on the CSV summary line: order,beam,tol,seed,score,layers,peak_layer_len.
 local layers
 layers="$(tail -1 "${scratch}/stdout.log" | cut -d, -f6)"
 [[ "$layers" =~ ^[0-9]+$ ]] || layers=0
 local nodes=$(( layers * beam ))
 local nps
 nps=$(awk -v n="$nodes" -v w="$wall" 'BEGIN{ if (w>0) printf "%.0f", n/w; else print 0 }')
 echo "run_producer: wall=${wall}s nodes=${nodes} nps=${nps} nps_unit=beam-nodes/s" >&2

 local url=""
 [[ -s "$emit" ]] && url="$(cat "$emit")"
 _rsa_verify_and_emit "$variant_csv" "$url" "$out_url"
 echo "nodes=${nodes} nps=${nps} nps_unit=beam-nodes/s wall_s=${wall}"
}

# ---------------------------------------------------------------------------
# 2. BLACKWOOD (blackwood_bt) -- depth-gated-break backtracker.
# Node-budget bounded, not time-bounded: run under `timeout budget_s` with
# a large per-restart nodecap and a big restart count so the portfolio
# just gets killed mid-restart at the wall clock. The binary checkpoints
# every new best to emit-dir/best_so_far.url as it goes (pattern),
# so a mid-restart SIGTERM still leaves a valid, scoreable board on disk
# -- this is why the "hard node cap" style still yields an honest ~60s
# wall-clock result.
# ---------------------------------------------------------------------------
run_blackwood() {
 local variant_csv="$1" seed="$2" budget_s="$3" out_url="$4"
 local scratch; scratch="$(_rsa_scratch "blackwood")"

 # nodecap=20M (not 200M): small enough that MANY restarts complete
 # within budget_s, so the binary's own periodic
 # `# progress N/1000 ... nodes=X ... Mnode/s ...` line (emitted every 25
 # restarts) gives an exact, binary-reported node count + throughput
 # right up to the last full 25-restart checkpoint before the timeout
 # kill -- see CALIBRATION.md for why 200M was too coarse (0-2 restarts
 # complete in 60s, so no progress line and no honest total-nodes figure
 # under a hard kill).
 timeout "$budget_s" "$BLACKWOOD_BIN" "$variant_csv" \
 --nodecap 20000000 --restarts 1000 --seed "$seed" \
 --emit-dir "$scratch" --emit-prefix bw \
 > "${scratch}/stdout.log" 2> "${scratch}/stderr.log"
 # timeout SIGTERMs the process at budget_s; rc 124 is the expected/normal
 # outcome here, not a failure -- the checkpoint file is what we bank.

 local prog_line nodes nps nps_mn
 prog_line="$(grep '^# progress' "${scratch}/stderr.log" | tail -1)"
 nodes="$(echo "$prog_line" | sed -n 's/.*nodes=\([0-9]*\).*/\1/p')"
 nps_mn="$(echo "$prog_line" | sed -n 's/.* \([0-9.]*\) Mnode\/s.*/\1/p')"
 [[ -z "$nodes" ]] && nodes=0
 if [[ -n "$nps_mn" ]]; then
 nps=$(awk -v m="$nps_mn" 'BEGIN{printf "%.0f", m*1e6}')
 else
 nps=0
 fi
 echo "run_blackwood: nodes=${nodes} nps=${nps} nps_unit=search-nodes/s (from: ${prog_line:-no progress line})" >&2

 local url=""
 [[ -s "${scratch}/best_so_far.url" ]] && url="$(cat "${scratch}/best_so_far.url")"
 _rsa_verify_and_emit "$variant_csv" "$url" "$out_url"
 echo "nodes=${nodes} nps=${nps} nps_unit=search-nodes/s"
}

# ---------------------------------------------------------------------------
# 3. VERHAARD (verhaard_faithful_v2) -- comb-order Verhaard-style
# backtracker. Same node-budget-bounded / timeout-wrapped / checkpoint
# pattern as run_blackwood (same author lineage, same checkpoint file
# convention).
# ---------------------------------------------------------------------------
run_verhaard() {
 local variant_csv="$1" seed="$2" budget_s="$3" out_url="$4"
 local scratch; scratch="$(_rsa_scratch "verhaard")"

 # Same nodecap=20M rationale as run_blackwood (see there).
 timeout "$budget_s" "$VERHAARD_BIN" "$variant_csv" \
 --nodecap 20000000 --restarts 1000 --seed "$seed" \
 --emit-dir "$scratch" --emit-prefix vh \
 > "${scratch}/stdout.log" 2> "${scratch}/stderr.log"

 local prog_line nodes nps nps_mn
 prog_line="$(grep '^# progress' "${scratch}/stderr.log" | tail -1)"
 nodes="$(echo "$prog_line" | sed -n 's/.*nodes=\([0-9]*\).*/\1/p')"
 nps_mn="$(echo "$prog_line" | sed -n 's/.* \([0-9.]*\) Mnode\/s.*/\1/p')"
 [[ -z "$nodes" ]] && nodes=0
 if [[ -n "$nps_mn" ]]; then
 nps=$(awk -v m="$nps_mn" 'BEGIN{printf "%.0f", m*1e6}')
 else
 nps=0
 fi
 echo "run_verhaard: nodes=${nodes} nps=${nps} nps_unit=search-nodes/s (from: ${prog_line:-no progress line})" >&2

 local url=""
 [[ -s "${scratch}/best_so_far.url" ]] && url="$(cat "${scratch}/best_so_far.url")"
 _rsa_verify_and_emit "$variant_csv" "$url" "$out_url"
 echo "nodes=${nodes} nps=${nps} nps_unit=search-nodes/s"
}

# ---------------------------------------------------------------------------
# 4. ALNS (alns_only) -- time-bounded local search, but needs a STARTING
# board. Fair 2-stage chain inside the SAME budget_s wall-clock window:
# stage A: run_producer's beam producer at a FIXED calibrated beam
# (16384, comb:14 order -- matches the memory-recorded winning
# pipeline "comb14 B=16384 seed9 -> 457 raw -> ALNS -> 461").
# This beam completes in ~9s regardless of budget_s (it is NOT
# time-bounded, it runs the fixed beam to completion) -- see
# CALIBRATION.md for the beam<->time table. A larger budget_s
# does NOT buy a wider beam here (that would need re-
# calibration); it buys MORE ALNS TIME, which is stage B's job.
# stage B: convert that board to alns_only's --cp-board JSON format
# (dump_board_json + a field rename `board`->`placement`;
# alns_only's load_cp_board wants top-level "placement") and
# spend the ACTUAL REMAINING wall-clock (budget_s minus
# stage-A's MEASURED wall time minus a small fixed conversion
# overhead) inside alns_only, which IS natively time-bounded
# via --alns-budget-ms. Measuring stage A's actual time (rather
# than assuming a fixed sub-budget) avoids wasting budget when
# the producer finishes faster than allotted.
# IMPORTANT: alns_only's load_cp_board hardcodes the piece catalog path
# `../data/puzzles/size_16_official_eternity.csv` (relative to CWD) to
# reconstruct piece ids/edges when parsing the cp-board JSON -- piece
# ids/edges are IDENTICAL across all 10 corner-pinned variants (only the
# 8 hint cells differ), so this is safe as long as this script's cwd
# resolves `../data/...` to the real repo `data/puzzles/` dir. We `cd`
# into $V2_ROOT for the alns_only invocation to guarantee that,
# regardless of the caller's cwd.
# ---------------------------------------------------------------------------
run_alns() {
 local variant_csv="$1" seed="$2" budget_s="$3" out_url="$4"
 local scratch; scratch="$(_rsa_scratch "alns")"

 local convert_overhead_s=2

 # --- stage A: producer base board (fixed calibrated beam) ------------
 local beam=16384
 local prod_emit="${scratch}/base.url"
 local ta0 ta1 prod_wall
 ta0=$(date +%s.%N)
 timeout 30 "$PRODUCER_BIN" "$variant_csv" \
 --order comb:14 --beam "$beam" --tol 0 --seed "$seed" \
 --emit-best "$prod_emit" \
 > "${scratch}/producer_stdout.log" 2> "${scratch}/producer_stderr.log"
 ta1=$(date +%s.%N)
 prod_wall=$(awk -v a="$ta0" -v b="$ta1" 'BEGIN{printf "%.2f", b-a}')
 echo "run_alns: stage A producer (beam=${beam}) wall=${prod_wall}s" >&2

 # Remaining wall-clock for ALNS = budget_s - measured stage-A wall -
 # fixed conversion overhead. Clamp to a sane floor so a slow stage A on
 # a loaded machine still leaves ALNS something to do.
 local prod_wall_int alns_budget_s alns_budget_ms
 prod_wall_int=$(awk -v w="$prod_wall" 'BEGIN{printf "%d", w+0.999}') # ceil
 alns_budget_s=$(( budget_s - prod_wall_int - convert_overhead_s ))
 (( alns_budget_s < 5 )) && alns_budget_s=5
 alns_budget_ms=$(( alns_budget_s * 1000 ))

 if [[ ! -s "$prod_emit" ]]; then
 echo "run_alns: stage A (producer base board) produced nothing" >&2
 _rsa_verify_and_emit "$variant_csv" "" "$out_url"
 return 1
 fi
 local base_url; base_url="$(cat "$prod_emit")"
 local base_score_line
 base_score_line="$("$VERIFY_BIN" "$variant_csv" "$base_url" 2>/dev/null | grep '^board\[0\]')"
 echo "run_alns: stage A base board -- ${base_score_line}" >&2

 # --- convert bucas URL -> alns_only's --cp-board JSON ("placement") --
 # dump_board_json/alns_only both hardcode ../data/puzzles/... relative
 # to CWD, so this whole stage runs with cwd=$V2_ROOT.
 local cpboard_json="${scratch}/base_cpboard.json"
 ( cd "$V2_ROOT" && "$DUMP_BOARD_JSON_BIN" "$base_url" ) \
 > "${scratch}/dump.json" 2> "${scratch}/dump_stderr.log"
 if [[ ! -s "${scratch}/dump.json" ]]; then
 echo "run_alns: dump_board_json failed, see ${scratch}/dump_stderr.log" >&2
 _rsa_verify_and_emit "$variant_csv" "$base_url" "$out_url"
 return 1
 fi
 jq '{placement: .board}' "${scratch}/dump.json" > "$cpboard_json"

 # --- stage B: alns_only, time-bounded via --alns-budget-ms -----------
 # NOTE: alns_only's `bucas:` and `ALNS: elapsed=... iters=...` summary
 # lines are eprintln!'d (stderr), not stdout -- must grep alns_stderr.log,
 # not alns_stdout.log (alns_only's stdout is empty/unused in this mode).
 local alns_stderr="${scratch}/alns_stderr.log"
 ( cd "$V2_ROOT" && timeout "$(( alns_budget_s + 15 ))" "$ALNS_BIN" \
 --cp-board "$cpboard_json" \
 --puzzle "$variant_csv" \
 --alns-budget-ms "$alns_budget_ms" \
 --seed "$seed" \
 --ops basic_lkh \
 > "${scratch}/alns_stdout.log" 2> "$alns_stderr" )
 local rc=$?
 if [[ $rc -ne 0 ]]; then
 echo "run_alns: alns_only exited rc=$rc (see ${alns_stderr})" >&2
 fi

 local url
 url="$(grep '^bucas: ' "$alns_stderr" | tail -1 | sed 's/^bucas: //')"
 if [[ -z "$url" ]]; then
 echo "run_alns: alns_only produced no bucas url; falling back to stage A board" >&2
 url="$base_url"
 fi

 # Node accounting: alns_only has no "node" concept -- its unit of work is
 # a destroy+repair ITERATION. Parse `ALNS: elapsed=<s> iters=<n>` and
 # report nps as iters/s (unit is explicitly iters/s, not nodes/s).
 local alns_summary iters elapsed_alns nps
 alns_summary="$(grep '^ALNS: elapsed=' "$alns_stderr" | tail -1)"
 iters="$(echo "$alns_summary" | sed -n 's/.*iters=\([0-9]*\).*/\1/p')"
 elapsed_alns="$(echo "$alns_summary" | sed -n 's/.*elapsed=\([0-9.]*\)s.*/\1/p')"
 [[ -z "$iters" ]] && iters=0
 if [[ -n "$elapsed_alns" ]]; then
 nps=$(awk -v i="$iters" -v w="$elapsed_alns" 'BEGIN{ if (w>0) printf "%.2f", i/w; else print 0 }')
 else
 nps=0
 fi
 echo "run_alns: stage B ${alns_summary:-<no ALNS summary line>}" >&2
 echo "run_alns: nodes=${iters} nps=${nps} nps_unit=iters/s" >&2

 _rsa_verify_and_emit "$variant_csv" "$url" "$out_url"
 echo "nodes=${iters} nps=${nps} nps_unit=iters/s"
}
