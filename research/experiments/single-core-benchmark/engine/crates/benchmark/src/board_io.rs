// Re-exports the canonical board-dump types from `eternity2-export`.
// Kept for source-compatibility with existing `eternity2_benchmark::board_io::*`
// imports; new code should import from `eternity2_export` directly.

pub use eternity2_export::{read_dump, write_dump, DumpedBoard};
