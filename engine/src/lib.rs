//! Eternity II engine. Native crate + WASM bindings (wasm-bindgen).
//!
//! This is the **canonical** implementation: the website loads it compiled to
//! WebAssembly, and the from-scratch ports in other languages
//! (`engine-ports/`) are literal
//! translations of it. The algorithm — conventions, RNG, generator, the nine
//! cell-visit orders, the step-able backtracking solver, and the scorer — is
//! described once, language-agnostically, in
//! `engine-ports/ALGORITHM.md`. If you are new to this code, read that
//! document alongside these modules; they were written to be read together.
//! Each module below cross-references the relevant ALGORITHM.md section.
//!
//! JS boundary rules: no u64 (counters cross as f64), no clocks (the browser
//! times things), all data as plain serde-serialized objects.

#![forbid(unsafe_code)]

mod bitset;
pub mod break_solver;
pub mod generator;
pub mod official;
pub mod paths;
pub mod solver;
pub mod types;

pub use break_solver::BreakSolver;
pub use generator::{generate, generate_framed, generate_solved_framed};
pub use official::official_puzzle;
pub use paths::{build_path, PATH_KINDS};
pub use solver::{score_board, Solver, Status};
pub use types::{canonical, rotated, Color, Hint, Puzzle, BORDER};

use wasm_bindgen::prelude::*;

fn to_js<T: serde::Serialize>(v: &T) -> JsValue {
    serde_wasm_bindgen::to_value(v).expect("serialize")
}

#[wasm_bindgen(js_name = officialPuzzle)]
pub fn official_puzzle_js() -> JsValue {
    to_js(&official::official_puzzle())
}

#[wasm_bindgen(js_name = generatePuzzle)]
pub fn generate_puzzle_js(size: u8, colors: u8, seed: u32) -> JsValue {
    to_js(&generator::generate(size, colors, seed))
}

/// Pieces in solution order/orientation: piece i belongs at cell i, rot 0.
#[wasm_bindgen(js_name = generateSolvedPuzzle)]
pub fn generate_solved_puzzle_js(size: u8, colors: u8, seed: u32) -> JsValue {
    to_js(&generator::generate_solved(size, colors, seed))
}

/// Like `generatePuzzle`, but when `framed` is true the frame-restricted colors
/// (`1..=min(5, colors-1)`) are confined to the border-band adjacencies and the
/// rest to the deep interior, mirroring real Eternity II. `framed=false` is
/// byte-for-byte identical to `generatePuzzle`.
#[wasm_bindgen(js_name = generatePuzzleFramed)]
pub fn generate_puzzle_framed_js(size: u8, colors: u8, seed: u32, framed: bool) -> JsValue {
    to_js(&generator::generate_framed(size, colors, seed, framed))
}

/// Solved-order counterpart of `generatePuzzleFramed`.
#[wasm_bindgen(js_name = generateSolvedPuzzleFramed)]
pub fn generate_solved_puzzle_framed_js(size: u8, colors: u8, seed: u32, framed: bool) -> JsValue {
    to_js(&generator::generate_solved_framed(size, colors, seed, framed))
}

#[wasm_bindgen(js_name = maxColors)]
pub fn max_colors_js(size: u8) -> u8 {
    generator::max_colors(size)
}

#[wasm_bindgen(js_name = pathKinds)]
pub fn path_kinds_js() -> Vec<String> {
    PATH_KINDS.iter().map(|s| (*s).to_string()).collect()
}

#[wasm_bindgen(js_name = buildPath)]
pub fn build_path_js(kind: &str, width: u8, height: u8, seed: u32) -> Result<Vec<u16>, JsValue> {
    paths::build_path(kind, width, height, seed)
        .ok_or_else(|| JsValue::from_str(&format!("unknown path kind: {kind}")))
}

#[wasm_bindgen(js_name = scoreBoard)]
pub fn score_board_js(puzzle: JsValue, board: Vec<i32>) -> Result<u32, JsValue> {
    let puzzle: Puzzle =
        serde_wasm_bindgen::from_value(puzzle).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(solver::score_board(&puzzle, &board))
}

#[wasm_bindgen]
pub struct WasmSolver {
    puzzle: Puzzle,
    inner: Solver,
    path: Vec<u16>,
    use_hints: bool,
    shuffle_pieces: bool,
    seed: u32,
}

#[wasm_bindgen]
impl WasmSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(
        puzzle: JsValue,
        path: Vec<u16>,
        use_hints: bool,
        shuffle_pieces: bool,
        seed: u32,
    ) -> Result<WasmSolver, JsValue> {
        let puzzle: Puzzle = serde_wasm_bindgen::from_value(puzzle)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let inner = Solver::new(&puzzle, &path, use_hints, shuffle_pieces, seed)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(WasmSolver {
            puzzle,
            inner,
            path,
            use_hints,
            shuffle_pieces,
            seed,
        })
    }

    /// Run up to `budget` placements/backtracks; returns a Report object.
    pub fn step(&mut self, budget: u32) -> JsValue {
        to_js(&self.inner.step(budget))
    }

    pub fn report(&self) -> JsValue {
        to_js(&self.inner.report())
    }

    /// Cell -> piece_id*4+rotation, or -1 while empty.
    pub fn board(&self) -> Vec<i32> {
        self.inner.board().to_vec()
    }

    #[wasm_bindgen(js_name = bestBoard)]
    pub fn best_board(&self) -> Vec<i32> {
        self.inner.best_board().to_vec()
    }

    pub fn score(&self) -> u32 {
        solver::score_board(&self.puzzle, self.inner.board())
    }

    #[wasm_bindgen(js_name = bestScore)]
    pub fn best_score(&self) -> u32 {
        solver::score_board(&self.puzzle, self.inner.best_board())
    }

    pub fn reset(&mut self) {
        self.inner = Solver::new(
            &self.puzzle,
            &self.path,
            self.use_hints,
            self.shuffle_pieces,
            self.seed,
        )
        .expect("reset with previously validated arguments");
    }
}

/// Break-tolerant solver: the same step-able surface, but a mismatch is allowed
/// only at the fixed `break_positions` cells (one each), plus a live `breaks`
/// count. This is the mechanism behind the community record solvers — see
/// `break_solver.rs`.
#[wasm_bindgen]
pub struct WasmBreakSolver {
    puzzle: Puzzle,
    inner: BreakSolver,
    path: Vec<u16>,
    use_hints: bool,
    break_positions: Vec<u16>,
}

#[wasm_bindgen]
impl WasmBreakSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(
        puzzle: JsValue,
        path: Vec<u16>,
        use_hints: bool,
        break_positions: Vec<u16>,
    ) -> Result<WasmBreakSolver, JsValue> {
        let puzzle: Puzzle = serde_wasm_bindgen::from_value(puzzle)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let inner = BreakSolver::new(&puzzle, &path, use_hints, &break_positions)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(WasmBreakSolver {
            puzzle,
            inner,
            path,
            use_hints,
            break_positions,
        })
    }

    pub fn step(&mut self, budget: u32) -> JsValue {
        to_js(&self.inner.step(budget))
    }

    pub fn report(&self) -> JsValue {
        to_js(&self.inner.report())
    }

    pub fn board(&self) -> Vec<i32> {
        self.inner.board().to_vec()
    }

    #[wasm_bindgen(js_name = bestBoard)]
    pub fn best_board(&self) -> Vec<i32> {
        self.inner.best_board().to_vec()
    }

    pub fn score(&self) -> u32 {
        solver::score_board(&self.puzzle, self.inner.board())
    }

    pub fn breaks(&self) -> u32 {
        self.inner.breaks_used()
    }

    pub fn reset(&mut self) {
        self.inner =
            BreakSolver::new(&self.puzzle, &self.path, self.use_hints, &self.break_positions)
                .expect("reset with previously validated arguments");
    }
}
