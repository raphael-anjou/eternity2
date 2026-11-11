//! Ladder instances: Owen colour counts, seeds, the hardness-peak generator
//! call with its scramble, the planted solution, and the board verifier.

use eternity2_engine::generator::XorShift;
use eternity2_engine::{generate_solved_framed_bc, rotated, Puzzle, BORDER};
use serde::{Deserialize, Serialize};

pub const SIZES: [u8; 5] = [6, 7, 8, 9, 10];
pub const FRAME_COLOURS: u8 = 5;
pub const TEST_SEEDS: std::ops::RangeInclusive<u32> = 1..=30;
pub const CALIBRATION_SEEDS: std::ops::RangeInclusive<u32> = 1001..=1010;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ladder {
    /// Nearest integer to the Owen criterion.
    Primary,
    /// The other adjacent integer.
    Secondary,
    /// 8x8 only at c = 10 (above critical; round-2 rung R8, file `8-c10`).
    EightC10,
}

impl Ladder {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::EightC10 => "8-c10",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "primary" => Some(Self::Primary),
            "secondary" => Some(Self::Secondary),
            "8-c10" => Some(Self::EightC10),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Test,
    Calibration,
}

impl Role {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Test => "test",
            Self::Calibration => "calibration",
        }
    }

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "test" => Some(Self::Test),
            "calibration" => Some(Self::Calibration),
            _ => None,
        }
    }

    #[must_use]
    pub fn seeds(self) -> Vec<u32> {
        match self {
            Self::Test => TEST_SEEDS.collect(),
            Self::Calibration => CALIBRATION_SEEDS.collect(),
        }
    }
}

/// Owen's one-expected-solution interior colour count,
/// I = (P! * 4^P)^(1/(2P)), P = (n-2)^2 (same formula as `hardness_peak
/// --criterion`).
#[must_use]
pub fn owen_criterion(n: u8) -> f64 {
    let p = f64::from((n - 2) * (n - 2));
    let ln_fact: f64 = (1..=((n - 2) as u32 * (n - 2) as u32)).map(|k| f64::from(k).ln()).sum();
    ((ln_fact + p * 4f64.ln()) / (2.0 * p)).exp()
}

/// Preregistered interior colour count for `ladder` at size `n`.
#[must_use]
pub fn colours(ladder: Ladder, n: u8) -> u8 {
    if ladder == Ladder::EightC10 {
        return 10;
    }
    let i = owen_criterion(n);
    let nearest = (i + 0.5).floor();
    let pick = match ladder {
        Ladder::Primary => nearest,
        Ladder::EightC10 => unreachable!(),
        Ladder::Secondary => {
            if nearest > i {
                i.floor()
            } else {
                i.ceil()
            }
        }
    };
    pick as u8
}

/// One emitted instance. `puzzle` is what solvers see; `planted` is for
/// verification only and is never passed to a solver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LadderInstance {
    pub id: String,
    pub ladder: String,
    pub role: String,
    pub n: u8,
    pub c: u8,
    pub seed: u32,
    pub owen_criterion: f64,
    pub generator: String,
    pub puzzle: Puzzle,
    pub planted: Vec<i32>,
}

/// The hardness-peak instance: `generate_solved_framed_bc(n, c + 5, seed,
/// true, 5)` scrambled with `XorShift(seed ^ 0xA5A5_A5A5)` (shuffle, then one
/// random rotation per piece). Returns the scrambled puzzle and the planted
/// board (row-major `piece * 4 + rot`).
#[must_use]
pub fn generate(n: u8, c: u8, seed: u32) -> (Puzzle, Vec<i32>) {
    let solved = generate_solved_framed_bc(n, c + FRAME_COLOURS, seed, true, FRAME_COLOURS);
    let cells = solved.pieces.len();
    let mut rng = XorShift::new(seed ^ 0xA5A5_A5A5);
    // Shuffling the index vector consumes the RNG exactly as shuffling the
    // pieces does, so the scrambled board is identical to hardness_peak's.
    let mut order: Vec<usize> = (0..cells).collect();
    rng.shuffle(&mut order);
    let mut puzzle = solved.clone();
    let mut planted = vec![-1i32; cells];
    for (j, &cell) in order.iter().enumerate() {
        let r = rng.below(4) as u8;
        puzzle.pieces[j] = rotated(solved.pieces[cell], r);
        planted[cell] = (j as i32) * 4 + i32::from((4 - r) % 4);
    }
    puzzle.name = format!("hard-ladder n{n} c{c} s{seed}");
    puzzle.hints.clear();
    (puzzle, planted)
}

#[must_use]
pub fn make_instance(ladder: Ladder, role: Role, n: u8, seed: u32) -> LadderInstance {
    let c = colours(ladder, n);
    let (puzzle, planted) = generate(n, c, seed);
    LadderInstance {
        id: format!("n{n}-c{c}-s{seed}"),
        ladder: ladder.name().into(),
        role: role.name().into(),
        n,
        c,
        seed,
        owen_criterion: owen_criterion(n),
        generator: format!(
            "generate_solved_framed_bc({n}, {}, {seed}, true, {FRAME_COLOURS}) + XorShift({seed} ^ 0xA5A5A5A5) scramble",
            c + FRAME_COLOURS
        ),
        puzzle,
        planted,
    }
}

#[must_use]
pub fn ladder_instances(ladder: Ladder, role: Role) -> Vec<LadderInstance> {
    let mut v = Vec::new();
    let sizes: &[u8] = if ladder == Ladder::EightC10 { &[8] } else { &SIZES };
    for &n in sizes {
        for seed in role.seeds() {
            v.push(make_instance(ladder, role, n, seed));
        }
    }
    v
}

/// Independent check of a full board: every piece once, grey exactly on the
/// rim, every interior seam matching.
pub fn verify_board(puzzle: &Puzzle, board: &[i32]) -> Result<(), String> {
    let (w, h) = (puzzle.width as usize, puzzle.height as usize);
    let cells = w * h;
    if board.len() != cells || puzzle.pieces.len() != cells {
        return Err(format!("board has {} cells, expected {cells}", board.len()));
    }
    let mut used = vec![false; cells];
    let mut edges = Vec::with_capacity(cells);
    for (pos, &v) in board.iter().enumerate() {
        if v < 0 {
            return Err(format!("cell {pos} empty"));
        }
        let (p, r) = ((v / 4) as usize, (v % 4) as u8);
        if p >= cells || used[p] {
            return Err(format!("piece {p} invalid or reused at cell {pos}"));
        }
        used[p] = true;
        edges.push(rotated(puzzle.pieces[p], r));
    }
    for y in 0..h {
        for x in 0..w {
            let [t, r, b, l] = edges[y * w + x];
            if (y == 0) != (t == BORDER)
                || (y == h - 1) != (b == BORDER)
                || (x == 0) != (l == BORDER)
                || (x == w - 1) != (r == BORDER)
            {
                return Err(format!("rim violated at ({x},{y})"));
            }
            if x + 1 < w && r != edges[y * w + x + 1][3] {
                return Err(format!("seam mismatch right of ({x},{y})"));
            }
            if y + 1 < h && b != edges[(y + 1) * w + x][0] {
                return Err(format!("seam mismatch below ({x},{y})"));
            }
        }
    }
    Ok(())
}

/// Whether an oriented piece agrees with the rim and with the assigned
/// neighbours on `board` (row-major `piece * 4 + rot`, `-1` empty).
#[must_use]
pub fn consistent(puzzle: &Puzzle, board: &[i32], pos: usize, e: [u8; 4]) -> bool {
    let (w, h) = (puzzle.width as usize, puzzle.height as usize);
    let (x, y) = (pos % w, pos / w);
    let [t, r, b, l] = e;
    if (y == 0) != (t == BORDER)
        || (y == h - 1) != (b == BORDER)
        || (x == 0) != (l == BORDER)
        || (x == w - 1) != (r == BORDER)
    {
        return false;
    }
    let side = |nb: usize, s: usize| -> Option<u8> {
        let v = board[nb];
        (v >= 0).then(|| rotated(puzzle.pieces[(v / 4) as usize], (v % 4) as u8)[s])
    };
    (y == 0 || side(pos - w, 2).is_none_or(|c| c == t))
        && (y == h - 1 || side(pos + w, 0).is_none_or(|c| c == b))
        && (x == 0 || side(pos - 1, 1).is_none_or(|c| c == l))
        && (x == w - 1 || side(pos + 1, 3).is_none_or(|c| c == r))
}
