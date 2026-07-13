// Re-exports the canonical reporting utilities from `eternity2-export`.
// Kept for source-compatibility with existing `eternity2_benchmark::report::*`
// imports; new code should import from `eternity2_export` directly.

pub use eternity2_export::{
    board_to_bucas_edges, bucas_url, puzzle_name_from_path, write_report, RunReport,
};
