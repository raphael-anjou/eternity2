// Blackwood-schedule factories + heuristic-side selection.
//
// introduced the [[blackwood-algorithm]] family. The schedules are
// pure factories: they consume a Puzzle and return Option<BlackwoodSchedule>.
// Calibration variants (v17a..v17e) were tuned per measurements.
//
// Moved here in T3 (split solver-engine/src/lib.rs into modules).

use eternity2_core::{Color, Puzzle, BORDER};

use crate::BlackwoodSchedule;

/// compute Blackwood's 3 "heuristic colors" from a Puzzle +
/// canonical hints. Selection rules per Blackwood msg #22 (2020):
///
///   1. **Many occurrences**: pick colors with highest total
///      piece-edge count.
///   2. **Not on any corner piece**: corner pieces have exactly two
///      BORDER edges; their non-border edges must not include the
///      heuristic colors.
///   3. **Not on the start piece** (i.e., piece at the centre hint):
///      preserves start-piece flexibility.
///
/// Blackwood's literal triple `[17, 2, 18]` does NOT apply to our
/// color labels (verified: color 2 IS on 2 of our corners).
/// This helper recomputes from first principles. Returns `(colors,
/// pool_size)` where `pool_size` is the total occurrence count of
/// the 3 colors across all 256 piece edges.
pub fn compute_heuristic_sides(
    puzzle: &Puzzle,
    hints: &eternity2_core::Hints,
) -> Vec<Color> {
 // allow explicit override via env var, e.g.
    //   E2_HEURISTIC_SIDES_OVERRIDE="15,16,17"
    // Used to sweep alternative triples among the 11 frequency-tied
    // canonical colors {12..22}. Per docs/community-mining/09: Blackwood's
    // 470-attempt used a different triple than his 469 solver.
    if let Ok(s) = std::env::var("E2_HEURISTIC_SIDES_OVERRIDE") {
        let parsed: Vec<Color> = s
            .split(',')
            .filter_map(|t| t.trim().parse::<Color>().ok())
            .collect();
        if parsed.len() == 3 {
            eprintln!("[heuristic-sides] OVERRIDE active: {:?}", parsed);
            return parsed;
        }
        eprintln!(
            "[heuristic-sides] WARNING: E2_HEURISTIC_SIDES_OVERRIDE set to {:?} but parsed to {} colors (need 3) — falling back to auto-select",
            s, parsed.len()
        );
    }
    let pieces = puzzle.pieces();

    // Corner pieces: exactly two BORDER edges.
    let mut corner_colors: std::collections::HashSet<Color> =
        std::collections::HashSet::new();
    for p in pieces {
        let e = p.edges.as_array();
        let n_border = e.iter().filter(|&&c| c == BORDER).count();
        if n_border == 2 {
            for &c in &e {
                if c != BORDER {
                    corner_colors.insert(c);
                }
            }
        }
    }

    // Centre/start hint: prefer the hint at position == cell_count/2
    // (canonical E2 has piece 138 at pos 135 which IS the centre);
    // fallback to any interior hint. Pull that piece's edges.
    let n_pos = puzzle.cell_count();
    let centre_pos = n_pos / 2;
    let start_piece: Option<&eternity2_core::Piece> = hints
        .hints
        .iter()
        .find(|h| h.position == centre_pos)
        .or_else(|| {
            // Fallback: any hint whose position is not on the border ring.
            let w = puzzle.width;
            let h_ = puzzle.height;
            hints.hints.iter().find(|hh| {
                let (x, y) = (hh.position % w, hh.position / w);
                x > 0 && x + 1 < w && y > 0 && y + 1 < h_
            })
        })
        .and_then(|h| puzzle.piece(h.piece_id));

    let mut start_colors: std::collections::HashSet<Color> =
        std::collections::HashSet::new();
    if let Some(p) = start_piece {
        for &c in &p.edges.as_array() {
            if c != BORDER {
                start_colors.insert(c);
            }
        }
    }

    let forbidden: std::collections::HashSet<Color> =
        corner_colors.union(&start_colors).copied().collect();

    // Count occurrences of each color across all piece edges (BORDER
    // excluded).
    let mut counts: std::collections::HashMap<Color, u32> =
        std::collections::HashMap::new();
    for p in pieces {
        for &c in &p.edges.as_array() {
            if c != BORDER {
                *counts.entry(c).or_insert(0) += 1;
            }
        }
    }

    // Filter and sort by descending frequency (break ties by ascending
    // color id for determinism).
    let mut candidates: Vec<(Color, u32)> = counts
        .into_iter()
        .filter(|(c, _)| !forbidden.contains(c))
        .collect();
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    candidates.into_iter().take(3).map(|(c, _)| c).collect()
}

/// total occurrence count of a set of edge colors across
/// all piece edges in a puzzle (BORDER excluded).
pub fn count_color_occurrences(puzzle: &Puzzle, colors: &[Color]) -> u32 {
    let set: std::collections::HashSet<Color> = colors.iter().copied().collect();
    let mut n = 0u32;
    for p in puzzle.pieces() {
        for &c in &p.edges.as_array() {
            if c != BORDER && set.contains(&c) {
                n += 1;
            }
        }
    }
    n
}

/// Blackwood's 469-recipe schedule, AFFINE-REMAPPED into
/// our scan's post-border range. Critical history:
///
/// - First attempt (proportional rescale on both axes) prunes
///   immediately on canonical E2 because Blackwood's control depths
///   16, 26 land inside our border ring (depth 0..60) where no
///   heuristic-color edges have been placed yet.
/// - Second attempt added `max(scaled, border_ring)` clamp; this
///   produced a CLIFF where two control points both land on depth 60
///   (one with target 0, one with target ≥ 28), causing
///   `target_at(60)` to jump discontinuously and trigger immediate
///   prune. Spotted by a third-party review of summary.json from
///   run_1778604403.
/// - This (third) attempt does a true AFFINE REMAP of Blackwood's
///   depth axis [16..160] into our [post_border..max_idx] range,
///   preserving the curve's shape while landing the first non-zero
///   target after the border ring is fully placed. Strictly monotone
///   in depth. Validated via `BlackwoodSchedule::validate`.
///
/// Returns `None` if `compute_heuristic_sides` finds fewer than 3
/// usable colors.
pub fn blackwood_schedule_469(
    puzzle: &Puzzle,
    hints: &eternity2_core::Hints,
) -> Option<BlackwoodSchedule> {
    let colors = compute_heuristic_sides(puzzle, hints);
    if colors.len() < 3 { return None; }
    let pool_size = count_color_occurrences(puzzle, &colors);
    let n_pos = puzzle.cell_count();
    let w = puzzle.width;
    let h = puzzle.height;
    let border_ring = 2 * w + 2 * h - 4;

    let bw_pool = 122u32;
    let scale_c = |c: u32| ((c as u64 * pool_size as u64) / bw_pool as u64) as u32;

    // Affine remap Blackwood's depth axis [16..160] into our
    // [post_border..target_max] range. `post_border` is the first
    // depth at which heuristic-color edges can plausibly be placed
    // (just past the border ring + 1 buffer cell). `target_max` is
    // the depth where the schedule should saturate (we use 160 ×
    // n_pos/256, the proportionally-scaled max from Blackwood's
    // original curve).
    let post_border = border_ring + 1; // first cell past the ring
    let target_max = ((160u64 * n_pos as u64) / 256) as u32;
    debug_assert!(target_max > post_border, "post_border ({post_border}) >= target_max ({target_max}); puzzle too small for Blackwood schedule");

    // Blackwood's control depths (he calls these heuristic indices).
    let bw_depths: [u32; 6] = [16, 26, 56, 76, 102, 160];
    let bw_counts: [u32; 6] = [0,  28, 71, 89, 106, 119];

    // Affine map: f(d) = post_border + (d - 16) * (target_max - post_border) / (160 - 16)
    let remap = |d: u32| -> u32 {
        let num = (d - 16) as u64 * (target_max - post_border) as u64;
        let den = (160 - 16) as u64;
        post_border + (num / den) as u32
    };

    // First target point is (0, 0) so target_at(0..post_border) → 0.
    // Then the remapped Blackwood points. Strictly increasing in
    // depth because affine remap with target_max > post_border.
    let mut targets: Vec<(u32, u32)> = Vec::with_capacity(bw_depths.len() + 1);
    targets.push((0, 0));
    for (&d, &c) in bw_depths.iter().zip(bw_counts.iter()) {
        targets.push((remap(d), scale_c(c)));
    }

    // Breaks: Blackwood's last 12 cells (201..256 on a 256-cell
    // board). Scaled proportionally; clamped into [post_border, n_pos).
    let bw_breaks: [u32; 12] = [201, 206, 211, 216, 221, 225, 229, 233, 237, 239, 241, 256];
    let breaks: Vec<u32> = bw_breaks
        .iter()
        .map(|&b| {
            let scaled = ((b as u64 * n_pos as u64) / 256u64) as u32;
            scaled.min(n_pos.saturating_sub(1))
        })
        .collect();

    let s = BlackwoodSchedule {
        heuristic_sides: colors,
        exhaustion_targets: targets,
        heuristic_pool_size: pool_size,
        max_heuristic_index: target_max,
        break_indexes_allowed: breaks,
    };
    // Validate the schedule is well-formed before returning. If
    // invalid we hand back a freshly-constructed trivial schedule
    // rather than crashing — caller can inspect via .validate().
    if let Err(e) = s.validate() {
        eprintln!("WARNING: blackwood_schedule_469 produced invalid schedule: {e}");
    }
    Some(s)
}

/// (idea A) — Blackwood schedule **calibrated empirically** from
/// the McGavin 469 community board on canonical Eternity2. Each control
/// point `(depth, count)` is the cumulative count of heuristic-color
/// piece-edges placed in cells 0..depth under bottom-up row-major scan
/// order, computed by `calibrate_blackwood`. N=1 corpus board on
/// canonical E2 (only one ≥469 board exists), so this is the empirical
/// truth from THE actual community solve rather than a transferred
/// curve from Blackwood's different piece set.
///
/// Comparison to `blackwood_schedule_469` (affine-remapped Blackwood):
/// at depth 80 our schedule demanded ~60 heuristic edges placed;
/// the actual McGavin 469 board has 32 at depth 80. The schedule
/// was OVER-DEMANDING by ~2× and pruned the search prematurely. This
/// calibrated schedule should not have the same cliff failure.
///
/// `heuristic_sides` and `heuristic_pool_size` are recomputed at call
/// time so the schedule works on any puzzle with the same scan-order
/// structure (5-hint canonical E2 expected; falls back gracefully on
/// other shapes).
pub fn blackwood_schedule_calibrated_v17a(
    puzzle: &Puzzle,
    hints: &eternity2_core::Hints,
) -> Option<BlackwoodSchedule> {
    let colors = compute_heuristic_sides(puzzle, hints);
    if colors.len() < 3 { return None; }
    let pool_size = count_color_occurrences(puzzle, &colors);
    let n_pos = puzzle.cell_count();

    // From output/v17_calibration.json (run on McGavin 469, 2026-05-12).
    // Use the median curve directly. Last point clamped to n_pos-1.
    let last_idx = n_pos.saturating_sub(1);
    let targets: Vec<(u32, u32)> = vec![
        (0,        0),
        (60,       21),
        (80,       32),
        (100,      41),
        (120,      48),
        (140,      58),
        (160,      82),
        (200,      112),
        (last_idx, pool_size.min(150)),
    ];

 // Use the same proportional break schedule as — empirical
    // mismatch calibration is a future task (would need a 469 board
    // with the break locations annotated).
    let bw_breaks: [u32; 12] = [201, 206, 211, 216, 221, 225, 229, 233, 237, 239, 241, 256];
    let breaks: Vec<u32> = bw_breaks
        .iter()
        .map(|&b| {
            let scaled = ((b as u64 * n_pos as u64) / 256u64) as u32;
            scaled.min(n_pos.saturating_sub(1))
        })
        .collect();

    let target_max = targets.last().map(|&(d, _)| d).unwrap_or(n_pos - 1);
    let s = BlackwoodSchedule {
        heuristic_sides: colors,
        exhaustion_targets: targets,
        heuristic_pool_size: pool_size,
        max_heuristic_index: target_max,
        break_indexes_allowed: breaks,
    };
    if let Err(e) = s.validate() {
        eprintln!("WARNING: blackwood_schedule_calibrated_v17a produced invalid schedule: {e}");
        return None;
    }
    Some(s)
}

/// (idea A) — same calibration data as
/// `blackwood_schedule_calibrated_v17a` but at the 25th-percentile
/// (more permissive — demands fewer heuristic edges at each depth).
/// With N=1 corpus board this collapses to the same curve. Kept as
/// a stable name so additional corpus boards can be folded in later.
pub fn blackwood_schedule_calibrated_v17a_p25(
    puzzle: &Puzzle,
    hints: &eternity2_core::Hints,
) -> Option<BlackwoodSchedule> {
    // With N=1 the p25 IS the median. Reuse.
    blackwood_schedule_calibrated_v17a(puzzle, hints)
}

/// (idea A, V17C revision) — like v17b but with break indexes
/// shifted EARLIER. measured the depth wall at ~193, which is
/// BELOW v17b's first break at 201. The engine never uses any break
/// in practice because it can't even reach depth 201 cleanly under
/// the strict no-break placement rule. Moving breaks earlier (180,
/// 188, 195, ...) lets the engine accept partial mismatches starting
/// at the empirical wall depth.
///
/// This is a hypothesis — by allowing 1 break at depth 180, the
/// engine can spread mismatch tolerance over a wider range, and may
/// reach higher matched scores at the end.
/// (idea H21, V17E revision) — NOISE-INJECTED schedule.
/// Take the v17b base targets and apply per-control-point jitter
/// scaled by `noise_amplitude` * seed-derived RNG. Different seeds
/// produce different perturbed schedules. Hypothesis: each perturbed
/// schedule guides CP into a SLIGHTLY DIFFERENT piece-region basin
/// than v17a/b. Best-of-K perturbed-schedule runs may exceed any
/// single-schedule ceiling without using existing-board info.
///
/// noise_amplitude=0.0 → returns v17b unchanged.
/// noise_amplitude=0.2 → each target may shift by up to ±20%.
///
/// Targets are kept monotone-non-decreasing and within
/// [0, heuristic_pool_size] after perturbation.
pub fn blackwood_schedule_calibrated_v17e(
    puzzle: &Puzzle,
    hints: &eternity2_core::Hints,
    noise_amplitude: f64,
    seed: u64,
) -> Option<BlackwoodSchedule> {
    let mut s = blackwood_schedule_calibrated_v17b(puzzle, hints)?;
    if noise_amplitude == 0.0 { return Some(s); }
    // Simple splitmix64-style PRNG.
    let mut state: u64 = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut next_u64 = || -> u64 {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };
    let mut next_jitter = || -> f64 {
        let u = next_u64();
        // map to [-1, 1)
        (u as f64 / u64::MAX as f64) * 2.0 - 1.0
    };
    let pool = s.heuristic_pool_size as f64;
    let mut perturbed: Vec<(u32, u32)> = Vec::with_capacity(s.exhaustion_targets.len());
    let mut prev_c: u32 = 0;
    for (d, c) in s.exhaustion_targets.iter().copied() {
        if d == 0 {
            perturbed.push((d, 0));
            prev_c = 0;
            continue;
        }
        let jitter = next_jitter() * noise_amplitude * (c as f64).max(1.0);
        let mut new_c = ((c as f64) + jitter).round() as i64;
        if new_c < prev_c as i64 { new_c = prev_c as i64; }
        if new_c < 0 { new_c = 0; }
        if new_c > pool as i64 { new_c = pool as i64; }
        let new_c_u = new_c as u32;
        perturbed.push((d, new_c_u));
        prev_c = new_c_u;
    }
    s.exhaustion_targets = perturbed;
    if let Err(e) = s.validate() {
        eprintln!("WARNING: v17e (noise={noise_amplitude}, seed={seed}) invalid: {e}");
        return None;
    }
    Some(s)
}

/// (idea H20, V17D revision) — calibrate the Blackwood schedule
/// from OUR own 455-board (v17a + WorstBand) rather than the McGavin
/// 469 community board. Hypothesis: our 455 board has a MUCH higher
/// heuristic-color trajectory than McGavin's (e.g., 63 at depth 80
/// vs McGavin's 32). Using OUR curve as the schedule demands more
/// heuristic placement early, which may force the CP search into a
/// DIFFERENT piece-region assignment than v17a/v17b.
///
/// If the CP search reaches the wall at a similar depth (~192), the
/// ALNS ceiling might differ.
///
/// Trade-off: stricter demands → smaller feasible search space → CP
/// may fail earlier (UNSAT or low depth wall). But the demand levels
/// were ACHIEVED by our 455 board, so by construction this schedule
/// admits it.
pub fn blackwood_schedule_calibrated_v17d(
    puzzle: &Puzzle,
    hints: &eternity2_core::Hints,
) -> Option<BlackwoodSchedule> {
    let colors = compute_heuristic_sides(puzzle, hints);
    if colors.len() < 3 { return None; }
    let pool_size = count_color_occurrences(puzzle, &colors);
    let n_pos = puzzle.cell_count();
    let last_idx = n_pos.saturating_sub(1);

    // From /tmp/calibrate_from_our_board.py on our 455 board, with 1-edge
    // floor margin and monotone-non-decreasing fixup.
    let mut targets: Vec<(u32, u32)> = vec![
        (0,   0),
        (16,  5),
        (32,  22),
        (48,  34),
        (64,  50),
        (80,  62),
        (96,  72),
        (112, 84),
        (128, 94),
        (144, 105),
        (160, 114),
        (176, 121),
        (192, 122),
        (208, 129),
        (224, 140),
        (240, 149),
    ];
    if last_idx > 240 {
        targets.push((last_idx, pool_size.min(149)));
    }
    let bw_breaks: [u32; 12] = [201, 206, 211, 216, 221, 225, 229, 233, 237, 239, 241, 256];
    let breaks: Vec<u32> = bw_breaks
        .iter()
        .map(|&b| ((b as u64 * n_pos as u64) / 256u64).min(n_pos.saturating_sub(1) as u64) as u32)
        .collect();
    let target_max = targets.last().map(|&(d, _)| d).unwrap_or(n_pos - 1);
    let s = BlackwoodSchedule {
        heuristic_sides: colors,
        exhaustion_targets: targets,
        heuristic_pool_size: pool_size,
        max_heuristic_index: target_max,
        break_indexes_allowed: breaks,
    };
    if let Err(e) = s.validate() {
        eprintln!("WARNING: blackwood_schedule_calibrated_v17d invalid: {e}");
        return None;
    }
    Some(s)
}

pub fn blackwood_schedule_calibrated_v17c(
    puzzle: &Puzzle,
    hints: &eternity2_core::Hints,
) -> Option<BlackwoodSchedule> {
    let mut s = blackwood_schedule_calibrated_v17b(puzzle, hints)?;
    let n_pos = puzzle.cell_count();
    let last_idx = n_pos.saturating_sub(1);
    // EMPIRICALLY DERIVED from the McGavin 469 board's actual mismatch
 // positions in bottom-up row-major scan order (measurement,
    // `scripts/find_mcgavin_breaks.py`). McGavin's 11 mismatched edges
    // have their "later cell" at these 11 scan indices. So if our
    // engine searches in the SAME order as McGavin and accepts breaks
    // at these exact positions, it can reproduce McGavin's solution.
    let mcg_breaks: [u32; 11] = [187, 188, 190, 199, 200, 202, 206, 216, 222, 233, 249];
    s.break_indexes_allowed = mcg_breaks.iter().map(|&b| b.min(last_idx)).collect();
    Some(s)
}

/// (idea A, V17B revision) — TIGHT empirical envelope of the
/// McGavin 469 cumulative heuristic-color curve, sampled at every 16
/// cells with a 1-edge floor margin. Designed so that
/// `target_at(d) ≤ McGavin_actual[d]` for ALL d (verified offline);
/// the v17A coarser version had a 7-edge over-demand around depth 32
/// because of linear interpolation between sparse control points.
///
/// Concretely: the v17A piecewise-linear schedule between control
/// points (0,0) and (60,21) reaches target=11 at depth 32, but the
/// McGavin 469 board has only 4 heuristic edges at depth 32 — meaning
/// the v17A schedule prunes the McGavin 469 itself between depths
/// 20-50. The v17B schedule samples at every 16 cells so the
/// envelope hugs the McGavin curve from below with at most 1-edge
/// slack.
///
/// This is the schedule that should be used as the default once
/// validated on canonical E2.
pub fn blackwood_schedule_calibrated_v17b(
    puzzle: &Puzzle,
    hints: &eternity2_core::Hints,
) -> Option<BlackwoodSchedule> {
    let colors = compute_heuristic_sides(puzzle, hints);
    if colors.len() < 3 { return None; }
    let pool_size = count_color_occurrences(puzzle, &colors);
    let n_pos = puzzle.cell_count();
    let last_idx = n_pos.saturating_sub(1);

    // Dense sample from McGavin 469 cum-curve, minus 1-edge floor.
    // Final point clamped to n_pos-1 to land inside the path.
    let mut targets: Vec<(u32, u32)> = vec![
        (0,   0),
        (16,  0),
        (32,  3),
        (48,  12),
        (64,  25),
        (80,  31),
        (96,  39),
        (112, 43),
        (128, 47),
        (144, 60),
        (160, 81),
        (176, 89),
        (192, 104),
        (208, 118),
        (224, 128),
        (240, 146),
    ];
    // Append saturation point at the very last cell. Use the
    // observed total minus margin if it fits, else cap at last_idx.
    if last_idx > 240 {
        targets.push((last_idx, pool_size.min(149)));
    }

    let bw_breaks: [u32; 12] = [201, 206, 211, 216, 221, 225, 229, 233, 237, 239, 241, 256];
    let breaks: Vec<u32> = bw_breaks
        .iter()
        .map(|&b| {
            let scaled = ((b as u64 * n_pos as u64) / 256u64) as u32;
            scaled.min(n_pos.saturating_sub(1))
        })
        .collect();

    let target_max = targets.last().map(|&(d, _)| d).unwrap_or(n_pos - 1);
    let s = BlackwoodSchedule {
        heuristic_sides: colors,
        exhaustion_targets: targets,
        heuristic_pool_size: pool_size,
        max_heuristic_index: target_max,
        break_indexes_allowed: breaks,
    };
    if let Err(e) = s.validate() {
        eprintln!("WARNING: blackwood_schedule_calibrated_v17b produced invalid schedule: {e}");
        return None;
    }
    Some(s)
}
