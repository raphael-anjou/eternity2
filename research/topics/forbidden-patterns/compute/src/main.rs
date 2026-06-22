//! Forbidden patterns in the official Eternity II piece set.
//!
//! Take a small shape of cells (two side by side, an L of three, a 2x2 square).
//! Pick that many *distinct* interior pieces and drop them into the shape, free
//! to rotate each one. The placement is "feasible" if every shared edge inside
//! the shape can be made to match for some choice of rotations, and "forbidden"
//! if no rotation choice works.
//!
//! This crate counts, exactly and exhaustively, how many distinct-piece
//! placements of each shape are feasible versus forbidden. There is no sampling
//! and no randomness, so the output reproduces byte-for-byte.
//!
//! Conventions come straight from the shared engine: edges are URDL
//! (up, right, down, left), rotation is clockwise quarter-turns, and color 0 is
//! the grey rim. Interior pieces are the 196 pieces with no grey edge — the only
//! ones that ever sit inside the board — so those are the population we use.

use eternity2_engine::official::official_puzzle;
use eternity2_engine::types::rotated;

type Edges = [u8; 4];

// Edge indices in URDL order.
const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

/// The four rotations of a piece, precomputed.
struct Rots([Edges; 4]);

impl Rots {
    fn new(e: Edges) -> Self {
        Rots([rotated(e, 0), rotated(e, 1), rotated(e, 2), rotated(e, 3)])
    }
}

/// Can two pieces sit side by side (left cell `a`, right cell `b`) with matching
/// colors across the shared vertical edge, for some rotation of each?
fn horizontally_feasible(a: &Rots, b: &Rots) -> bool {
    for ra in &a.0 {
        for rb in &b.0 {
            if ra[RIGHT] == rb[LEFT] {
                return true;
            }
        }
    }
    false
}

/// Can two pieces sit one above the other (top cell `a`, bottom cell `b`) with
/// matching colors across the shared horizontal edge, for some rotation of each?
fn vertically_feasible(a: &Rots, b: &Rots) -> bool {
    for ra in &a.0 {
        for rb in &b.0 {
            if ra[DOWN] == rb[UP] {
                return true;
            }
        }
    }
    false
}

/// Can three pieces fill an L (top-left, top-right, bottom-left) so the two
/// shared edges both match? The TL-TR edge is vertical, the TL-BL edge is
/// horizontal — the two constraints share only the TL piece, so they interact
/// through its rotation.
fn l_feasible(tl: &Rots, tr: &Rots, bl: &Rots) -> bool {
    for a in &tl.0 {
        let right_ok = tr.0.iter().any(|x| x[LEFT] == a[RIGHT]);
        let down_ok = bl.0.iter().any(|x| x[UP] == a[DOWN]);
        if right_ok && down_ok {
            return true;
        }
    }
    false
}

/// Can four pieces fill a 2x2 square (TL, TR, BL, BR) with all four internal
/// edges matching, for some rotation of each? Search rotations with early cuts.
fn square_feasible(tl: &Rots, tr: &Rots, bl: &Rots, br: &Rots) -> bool {
    for a in &tl.0 {
        for b in &tr.0 {
            if a[RIGHT] != b[LEFT] {
                continue;
            }
            for c in &bl.0 {
                if a[DOWN] != c[UP] {
                    continue;
                }
                for d in &br.0 {
                    if b[DOWN] == d[UP] && c[RIGHT] == d[LEFT] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// One shape's exact tally over all distinct-piece placements.
struct Tally {
    placements: u64,
    feasible: u64,
}

impl Tally {
    fn forbidden(&self) -> u64 {
        self.placements - self.feasible
    }
    fn forbidden_pct(&self) -> f64 {
        100.0 * self.forbidden() as f64 / self.placements as f64
    }
}

fn main() {
    let puzzle = official_puzzle();
    // The 196 interior pieces: no grey (color 0) edge.
    let interior: Vec<Rots> = puzzle
        .pieces
        .iter()
        .filter(|e| !e.contains(&0))
        .map(|&e| Rots::new(e))
        .collect();
    let n = interior.len();

    // Pair shapes: ordered distinct pairs.
    let mut horiz = Tally { placements: 0, feasible: 0 };
    let mut vert = Tally { placements: 0, feasible: 0 };
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            horiz.placements += 1;
            if horizontally_feasible(&interior[i], &interior[j]) {
                horiz.feasible += 1;
            }
            vert.placements += 1;
            if vertically_feasible(&interior[i], &interior[j]) {
                vert.feasible += 1;
            }
        }
    }

    // L-tromino: ordered distinct triples.
    let mut l = Tally { placements: 0, feasible: 0 };
    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            // Prune: TL-TR must be horizontally feasible at all.
            let tr_ok = horizontally_feasible(&interior[i], &interior[j]);
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }
                l.placements += 1;
                if tr_ok && vertically_feasible(&interior[i], &interior[k])
                    && l_feasible(&interior[i], &interior[j], &interior[k])
                {
                    l.feasible += 1;
                }
            }
        }
    }

    // 2x2 square: ordered distinct quadruples. Pruned by the pair feasibilities
    // of the two edges incident to the top-left piece, which collapses the
    // ~1.46e9 raw quadruples to a few-second exhaustive sweep.
    let mut sq = Tally { placements: 0, feasible: 0 };
    for a in 0..n {
        for b in 0..n {
            if b == a {
                continue;
            }
            let ab = horizontally_feasible(&interior[a], &interior[b]);
            for c in 0..n {
                if c == a || c == b {
                    continue;
                }
                let ac = vertically_feasible(&interior[a], &interior[c]);
                let prunable = !(ab && ac);
                for d in 0..n {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    sq.placements += 1;
                    if prunable {
                        continue;
                    }
                    if square_feasible(&interior[a], &interior[b], &interior[c], &interior[d]) {
                        sq.feasible += 1;
                    }
                }
            }
        }
    }

    // Emit a compact, stable JSON document (fields in fixed order, no floats
    // beyond a rounded percentage for display — the raw counts are authoritative).
    let row = |name: &str, cells: u32, t: &Tally| {
        format!(
            "    {{\n      \"shape\": \"{}\",\n      \"cells\": {},\n      \"placements\": {},\n      \"feasible\": {},\n      \"forbidden\": {},\n      \"forbiddenPct\": {:.4}\n    }}",
            name,
            cells,
            t.placements,
            t.feasible,
            t.forbidden(),
            t.forbidden_pct()
        )
    };
    let rows = [
        row("2-horizontal", 2, &horiz),
        row("2-vertical", 2, &vert),
        row("L-tromino", 3, &l),
        row("2x2", 4, &sq),
    ];
    println!("{{");
    println!("  \"pieceSet\": \"official Eternity II (256 pieces)\",");
    println!("  \"population\": \"interior pieces only (no grey edge)\",");
    println!("  \"interiorPieceCount\": {},", n);
    println!("  \"note\": \"Exact, exhaustive counts over distinct-piece placements. Feasible = some rotation of each piece makes every shared edge match.\",");
    println!("  \"shapes\": [");
    println!("{}", rows.join(",\n"));
    println!("  ]");
    println!("}}");
}
