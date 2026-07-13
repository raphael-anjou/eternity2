// Re-exports the canonical puzzle/CSV loader from `eternity2-puzzle-io`.
// Kept for source-compatibility with existing `eternity2_benchmark::loader::*`
// imports; new code should depend on `eternity2-puzzle-io` directly.

pub use eternity2_puzzle_io::{load_puzzle, load_puzzle_with_hints, LoadError};
