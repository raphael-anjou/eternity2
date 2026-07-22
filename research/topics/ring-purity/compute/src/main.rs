//! Ring Purity checker.
//!
//! Recomputes, from the official 256-piece instance alone, every clause of the
//! Ring Purity Theorem (vol-226 FLOW.md, Section 6) plus its two strength
//! measures. Deterministic clauses hard-fail (nonzero exit) on any mismatch;
//! the sequential-importance-sampling measure is reported for comparison
//! within sampling error. Emits one JSON object on stdout.

use e2_kit::{official_instance, XorShift};
use serde_json::json;
use std::process::ExitCode;

/// A border piece's contribution to the ring multigraph: its piece id and its
/// two ring-facing frame colors (equal for a self-loop).
#[derive(Clone, Copy)]
struct RingEdge {
    a: u8,
    b: u8,
}

fn fail(msg: &str) -> ExitCode {
    eprintln!("RING-PURITY CHECK FAILED: {msg}");
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let inst = official_instance(true);
    let pieces = &inst.pieces;

    // ---- Clause 1: piece-type histogram by gray-slot count ----------------
    let mut by_type = [0usize; 3]; // interior, edge, corner
    for (_, p) in pieces.iter() {
        let z = p.border_edge_count() as usize;
        if z > 2 {
            return fail("piece with more than 2 gray slots");
        }
        by_type[z] += 1;
    }
    if by_type != [196, 56, 4] {
        return fail(&format!("piece-type histogram {by_type:?} != [196, 56, 4]"));
    }

    // ---- Clause 2-4: frame colors and the half-edge histogram -------------
    // Half-edge counts per color, split by piece class.
    let mut cnt_total = [0u32; 256];
    let mut cnt_interior = [0u32; 256];
    for (_, p) in pieces.iter() {
        let interior = p.border_edge_count() == 0;
        for &c in &p.edges {
            cnt_total[c as usize] += 1;
            if interior {
                cnt_interior[c as usize] += 1;
            }
        }
    }
    // Frame colors: non-gray colors that never occur on an interior piece.
    let frame: Vec<u8> = (1u8..=255)
        .filter(|&c| cnt_total[c as usize] > 0 && cnt_interior[c as usize] == 0)
        .collect();
    if frame.len() != 5 {
        return fail(&format!("expected 5 frame colors, found {}: {frame:?}", frame.len()));
    }
    for &c in &frame {
        if cnt_total[c as usize] != 24 {
            return fail(&format!("frame color {c} has {} half-edges, expected 24", cnt_total[c as usize]));
        }
    }
    let mut nonframe_counts: Vec<u32> = (1u8..=255)
        .filter(|&c| cnt_total[c as usize] > 0 && !frame.contains(&c))
        .map(|c| cnt_total[c as usize])
        .collect();
    nonframe_counts.sort_unstable();
    let expected_nonframe: Vec<u32> =
        std::iter::repeat(48).take(5).chain(std::iter::repeat(50).take(12)).collect();
    if nonframe_counts != expected_nonframe {
        return fail(&format!("non-frame color histogram {nonframe_counts:?} != 5x48 + 12x50"));
    }
    let is_frame = |c: u8| frame.contains(&c);

    // ---- Clause 5-6: per-piece slot structure; collect ring edges ---------
    let mut ring_edges: Vec<RingEdge> = Vec::with_capacity(60);
    for (pid, p) in pieces.iter() {
        match p.border_edge_count() {
            0 => {}
            1 => {
                // Edge piece: gray at slot z; the slot opposite z must be the
                // single non-frame slot, the two adjacent slots frame-colored.
                let z = (0..4).find(|&i| p.edges[i] == 0).unwrap();
                let inward = p.edges[(z + 2) % 4];
                let r1 = p.edges[(z + 1) % 4];
                let r2 = p.edges[(z + 3) % 4];
                if is_frame(inward) {
                    return fail(&format!("edge piece {pid}: opposite-gray slot is frame-colored"));
                }
                if !is_frame(r1) || !is_frame(r2) {
                    return fail(&format!("edge piece {pid}: a gray-adjacent slot is not frame-colored"));
                }
                ring_edges.push(RingEdge { a: r1, b: r2 });
            }
            2 => {
                // Corner piece: the two gray slots must be cyclically adjacent
                // (Lemma 5.1) and both non-gray slots frame-colored.
                let zs: Vec<usize> = (0..4).filter(|&i| p.edges[i] == 0).collect();
                let adjacent = (zs[1] + 4 - zs[0]) % 4 == 1 || (zs[0] + 4 - zs[1]) % 4 == 1;
                if !adjacent {
                    return fail(&format!("corner piece {pid}: gray slots not cyclically adjacent"));
                }
                let cs: Vec<u8> =
                    (0..4).filter(|&i| p.edges[i] != 0).map(|i| p.edges[i]).collect();
                if !is_frame(cs[0]) || !is_frame(cs[1]) {
                    return fail(&format!("corner piece {pid}: a non-gray slot is not frame-colored"));
                }
                ring_edges.push(RingEdge { a: cs[0], b: cs[1] });
            }
            _ => unreachable!(),
        }
    }

    // ---- Clause 7: zero-slack saturation ----------------------------------
    let demand = 56 * 2 + 4 * 2;
    let supply: u32 = frame.iter().map(|&c| cnt_total[c as usize]).sum();
    if demand != 120 || supply != 120 {
        return fail(&format!("saturation mismatch: demand {demand}, supply {supply}"));
    }

    // ---- Clause 8-9: the ring multigraph M --------------------------------
    if ring_edges.len() != 60 {
        return fail(&format!("ring multigraph has {} edges, expected 60", ring_edges.len()));
    }
    let vid = |c: u8| frame.iter().position(|&f| f == c).unwrap();
    let mut degree = [0u32; 5];
    let mut loops = 0u32;
    let mut pairs = std::collections::HashSet::new();
    for e in &ring_edges {
        degree[vid(e.a)] += 1;
        degree[vid(e.b)] += 1;
        if e.a == e.b {
            loops += 1;
        }
        let (x, y) = (vid(e.a).min(vid(e.b)), vid(e.a).max(vid(e.b)));
        pairs.insert((x, y));
    }
    if degree != [24; 5] {
        return fail(&format!("multigraph degrees {degree:?} != [24; 5]"));
    }
    // Connectivity via union-find over the 5 vertices.
    let mut parent: Vec<usize> = (0..5).collect();
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            let r = find(parent, parent[x]);
            parent[x] = r;
        }
        parent[x]
    }
    for e in &ring_edges {
        let (ra, rb) = (find(&mut parent, vid(e.a)), find(&mut parent, vid(e.b)));
        parent[ra] = rb;
    }
    let root = find(&mut parent, 0);
    if (1..5).any(|v| find(&mut parent, v) != root) {
        return fail("ring multigraph is not connected");
    }
    // Connected + all degrees even => Eulerian circuit exists (Euler).
    let eulerian = true;
    if loops != 14 {
        return fail(&format!("{loops} self-loop pieces, expected 14"));
    }
    if pairs.len() != 15 {
        return fail(&format!("{} distinct color-pairs, expected 15", pairs.len()));
    }

    // ---- Clause 10: exact first-step branching (Measurement 6.3) ----------
    // Distinct border pieces carrying at least one ring-facing slot of color c.
    let mut incident = [0u32; 5];
    for e in &ring_edges {
        incident[vid(e.a)] += 1;
        if e.a != e.b {
            incident[vid(e.b)] += 1;
        }
    }
    let mut incident_sorted = incident;
    incident_sorted.sort_unstable();
    if incident_sorted != [20, 21, 21, 22, 22] {
        return fail(&format!("branching multiset {incident_sorted:?} != [20, 21, 21, 22, 22]"));
    }
    let mean_branch = f64::from(incident.iter().sum::<u32>()) / 5.0;
    let reduction = 59.0 / mean_branch;

    // ---- Clauses 11-13: SIS greedy ring completion (Measurement 6.4) ------
    let seed: u32 = std::env::args()
        .skip_while(|a| a != "--seed")
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let trials = 1000u32;
    let mut completed = 0u32;
    let mut closed = 0u32;
    let mut logps: Vec<f64> = Vec::new();
    let mut rng = XorShift::new(0x5EED_0000 ^ seed);
    for _ in 0..trials {
        let mut remaining: Vec<RingEdge> = ring_edges.clone();
        // First piece uniform; orientation uniform.
        let i = rng.next_below(remaining.len() as u32) as usize;
        let first = remaining.swap_remove(i);
        let (start, mut open) =
            if rng.next_below(2) == 0 { (first.a, first.b) } else { (first.b, first.a) };
        let mut logp = 0.0f64;
        let mut dead = false;
        for _ in 0..59 {
            let legal: Vec<usize> = (0..remaining.len())
                .filter(|&j| remaining[j].a == open || remaining[j].b == open)
                .collect();
            if legal.is_empty() {
                dead = true;
                break;
            }
            logp += (legal.len() as f64 / remaining.len() as f64).log10();
            let pick = legal[rng.next_below(legal.len() as u32) as usize];
            let e = remaining.swap_remove(pick);
            open = if e.a == open { e.b } else { e.a };
        }
        if !dead {
            completed += 1;
            logps.push(logp);
            if open == start {
                closed += 1;
            }
        }
    }
    let mean_logp = if logps.is_empty() { f64::NAN } else { logps.iter().sum::<f64>() / logps.len() as f64 };
    let sd_logp = if logps.len() < 2 {
        f64::NAN
    } else {
        (logps.iter().map(|x| (x - mean_logp).powi(2)).sum::<f64>() / (logps.len() - 1) as f64).sqrt()
    };
    // log10(59!) via lgamma-free exact sum.
    let log10_59fact: f64 = (2..=59u32).map(|k| f64::from(k).log10()).sum();

    let out = json!({
        "instance": "official 16x16, 256 pieces, 5 clues",
        "deterministic": {
            "piece_type_histogram": {"interior": by_type[0], "edge": by_type[1], "corner": by_type[2]},
            "frame_colors": frame,
            "frame_half_edges_each": 24,
            "frame_supply": supply,
            "ring_demand": demand,
            "zero_slack": supply == demand,
            "edge_pieces_two_frame_ring_slots": true,
            "edge_piece_nonframe_slot_opposite_gray": true,
            "corner_pieces_both_slots_frame": true,
            "multigraph": {
                "vertices": 5,
                "edges": ring_edges.len(),
                "degrees": degree,
                "connected": true,
                "eulerian": eulerian,
                "self_loops": loops,
                "distinct_color_pairs": pairs.len(),
            },
            "branching": {
                "incident_pieces_sorted": incident_sorted,
                "mean": mean_branch,
                "reduction_vs_59": reduction,
            },
        },
        "sis": {
            "seed": seed,
            "trials": trials,
            "completed_59_steps": completed,
            "completion_rate": f64::from(completed) / f64::from(trials),
            "closed_circuits": closed,
            "mean_log10_path_prob": mean_logp,
            "sd_log10_path_prob": sd_logp,
            "uniform_baseline_log10": -log10_59fact,
        },
        "expected": {
            "branching_multiset": [20, 21, 21, 22, 22],
            "reduction": 2.78,
            "completion_rate_source": 0.094,
            "mean_log10_source": -27.3,
            "uniform_baseline_source": -80.1,
        },
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    ExitCode::SUCCESS
}
