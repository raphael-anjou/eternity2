// Reporting + serialization utilities shared across all bench/benchmark
// binaries. Consolidates what used to be duplicated between
// `bench-audit`, `benchmark::report`, and `benchmark::board_io`.
//
// Scope: pure functions over (Puzzle, Board) + JSON I/O. No solver,
// no localsearch, no propagator state — those keep their own
// inlinable scoring helpers (e.g. `localsearch::alns::score_board`)
// because they live on the hot path.

#![forbid(unsafe_code)]

mod board_io;
mod bucas;
mod report;
mod score;
mod verify;

pub use board_io::{
    load_board, load_board_csv, read_dump, save_board, save_board_csv, write_dump,
    BoardMetadata, DumpedBoard, LoadError,
};
pub use bucas::{
    board_to_bucas_edges, bucas_url, decode_bucas_board, BucasDecode, BucasDecodeError,
};
pub use report::{puzzle_name_from_path, write_report, RunReport};
pub use score::{internal_edge_count, placed_count, render_board, score_board};
pub use verify::{
    verify, BorderSide, BorderViolation, ExpectedKind, HintCompliance, HintMismatch,
    VerifyReport,
};
