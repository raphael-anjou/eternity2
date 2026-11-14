#!/usr/bin/env bash
# Baseline tournament (PREREG.md): build, then run every entrant in the
# preregistered tier order. Existing result files are skipped, so a rerun
# resumes. DEADLINE (HH:MM, local) stops launching new entrants.
#   run.sh build | run | part LISTFILE | analyze | all
# `part` runs the "entrant seeds" lines of LISTFILE (used to move work to a
# second machine, see PREREG_AMENDMENTS.md).
set -euo pipefail
cd "$(dirname "$0")"
THREADS="${THREADS:-4}"
DEADLINE="${DEADLINE:-22:00}"
BIN=compute/target/release/tournament
TAG="${TAG:-}"   # log-file suffix on a second machine, e.g. TAG=-m1

build() { cargo build --release -j "$THREADS" --manifest-path compute/Cargo.toml; }

past_deadline() { [[ "$(date +%H:%M)" > "$DEADLINE" || "$(date +%H:%M)" == "$DEADLINE" ]]; }

go() { # entrant [seeds]
  if past_deadline; then echo "NOT REACHED (deadline $DEADLINE): $*" | tee -a "results/not-reached$TAG.txt"; return 0; fi
  local t0=$SECONDS
  "$BIN" run --entrant "$1" --seeds "${2:-1}" --threads "$THREADS" --out-dir results/raw 2>>"results/log$TAG.txt" </dev/null
  echo "$(date +%FT%T) $1 seeds=${2:-1} $((SECONDS - t0)) s" | tee -a "results/stage-times$TAG.txt"
}

STOCH=(scb-border_first_random mrv-rand scb-ad fc-luby ktam-b3 ktam-v1 ktam-v2 cmp-hall-frontier cmp-hall-all-k4 cmp-hall-all cmp-hall-frontier-k4)

runall() {
  mkdir -p results/raw
  # Tier 1
  go bb-mrv; go scb-gacolor_ac3_random 1,2,3; go hp-dfs
  # Tier 2
  for e in engine-dfs-spiral-in engine-dfs-border-first engine-dfs-snake scb-naive_rowmajor scb-naive_spiral \
           scb-border_first_lcv scb-border_first_full scb-border_first_random scb-rare_color_first example-dfs \
           scb-gacolor_ac3 scb-gacolor_ac3_ns1 scb-gacolor_ac3_lcv mrv-nohs mrv-bf mrv-rand scb-ad; do go "$e"; done
  # Tier 3
  for e in fc-dfs rm-dfs fc-luby probcut-v1 probcut-v2 ktam-b3 ktam-v1 ktam-v2 \
           cmp-hall-frontier cmp-hall-all-k4 cmp-hall-all cmp-hall-frontier-k4 bb-impact bb-learned bb-single; do go "$e"; done
  # Tier 4
  for e in engine-dfs-column-major engine-dfs-spiral-out engine-dfs-diagonal engine-dfs-double-snake engine-dfs-random \
           scb-border_first_parity scb-border_first_gacolor scb-chess_gacolor scb-gacolor_symbreak scb-chess_gacolor_ac3 \
           scb-joe_depth150 scb-joe_depth150_bp scb-joe_depth150_bp_rec scb-joe_depth150_bp_rec_layered scb-verhaard_preferred; do go "$e"; done
  # Tier 5: seeds 2 and 3 of the stochastic entrants
  for e in "${STOCH[@]}"; do go "$e" 2,3; done
  # Tier 6: check arms
  go scb-fork-off; go cmp-fork-off
  echo "$(date +%FT%T) done" | tee -a "results/stage-times$TAG.txt"
}

analyze() { python3 analysis/analyze.py; }

case "${1:-all}" in
  build) build ;;
  run) runall ;;
  part) mkdir -p results/raw; while read -r e sd; do [[ -z "$e" || "$e" == \#* ]] && continue; go "$e" "$sd"; done < "$2" ;;
  analyze) analyze ;;
  all) build; runall; analyze ;;
  *) echo "usage: $0 build|run|analyze|all"; exit 2 ;;
esac
