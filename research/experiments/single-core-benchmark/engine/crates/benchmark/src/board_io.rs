// Re-exports the canonical board I/O from `eternity2-export`. The legacy
// DumpedBoard / read_dump / write_dump shapes are gone; the one write path is
// the canonical `e2_io::BoardDoc` (via `save_board`), and `load_board` still
// reads every in-the-wild shape. New code should import from `eternity2_export`.

pub use eternity2_export::{load_board, save_board, BoardMetadata, LoadError};
