//! Instances. Generator: `hard_ladder::instance::generate(n, c, seed)`, which
//! is `generate_solved_framed_bc(n, c + 5, seed, true, 5)` followed by the
//! hardness-peak scramble (`XorShift(seed ^ 0xA5A5A5A5)` shuffle, one random
//! rotation per piece). `c` is the interior colour count; 5 frame colours.
//! The planted board is written beside the puzzle for verification only.

use eternity2_engine::Puzzle;
use serde::{Deserialize, Serialize};

pub const FRAME_COLOURS: u8 = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub role: String,
    pub n: u8,
    pub c: u8,
    pub seed: u32,
    /// Interior seams of an n x n board, 2 n (n - 1): the planted optimum.
    pub optimum: u32,
    pub generator: String,
    pub puzzle: Puzzle,
    pub planted: Vec<i32>,
}

#[must_use]
pub fn optimum(n: u8) -> u32 {
    let n = u32::from(n);
    2 * n * (n - 1)
}

#[must_use]
pub fn make(role: &str, n: u8, c: u8, seed: u32) -> Instance {
    let (mut puzzle, planted) = hard_ladder::instance::generate(n, c, seed);
    puzzle.name = format!("partial-match-ladder n{n} c{c} s{seed}");
    Instance {
        id: format!("n{n}-c{c}-s{seed}"),
        role: role.into(),
        n,
        c,
        seed,
        optimum: optimum(n),
        generator: format!(
            "hard_ladder::instance::generate({n}, {c}, {seed}) = generate_solved_framed_bc({n}, {}, {seed}, true, {FRAME_COLOURS}) + XorShift({seed} ^ 0xA5A5A5A5) scramble",
            c + FRAME_COLOURS
        ),
        puzzle,
        planted,
    }
}

pub fn load(path: &std::path::Path) -> Result<Vec<Instance>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
        .collect()
}

/// Regenerate every row and compare it with the committed one (puzzle,
/// planted board, optimum) and check the planted board verifies at the optimum.
pub fn check(rows: &[Instance]) -> Result<(), String> {
    for r in rows {
        let fresh = make(&r.role, r.n, r.c, r.seed);
        if fresh.puzzle.pieces != r.puzzle.pieces
            || fresh.planted != r.planted
            || fresh.optimum != r.optimum
            || fresh.puzzle.num_colors != r.puzzle.num_colors
        {
            return Err(format!("{}: differs from the generator", r.id));
        }
        let s = crate::verify::verify(&r.puzzle, &r.planted)?;
        if s != r.optimum {
            return Err(format!(
                "{}: planted board scores {s}, optimum {}",
                r.id, r.optimum
            ));
        }
    }
    Ok(())
}
