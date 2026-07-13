// Library facade for the benchmark crate. Exposes the loader so that
// auxiliary binaries (e.g. `official_e2`) can share the CSV parser.
pub mod board_io;
pub mod loader;
pub mod report;
pub mod runner;
