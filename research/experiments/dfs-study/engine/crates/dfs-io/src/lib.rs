//! The shared IO contract for the DFS study, plus lossless converters between
//! the formats the blog's other engines already speak.
//!
//! Every algorithm in the study consumes one [`Instance`] and emits one
//! [`SolveOutput`]. That is the whole contract. Around it sit converters so a
//! board or instance can move between any two blog engines without loss:
//!
//! * **site JSON** (`SiteInstance`): the schema the sibling `single-core-
//!   benchmark` variants are written in, byte-compatible with the site's
//!   `Puzzle` type. [`Instance::from_site_json`] reads the study's variants
//!   directly, so the two experiments run the *same* ten instances.
//! * **CSV**: the row-per-piece format the standalone community engines
//!   (McGavin's C, Blackwood's C#) read. [`load_puzzle_csv`] /
//!   [`Instance::to_csv`].
//! * **bucas URL**: the viewer format. [`SolveOutput`] carries one for every
//!   board, so every result is verifiable in `/viewer`.
//! * **hints / clues**: pinned placements, expressed uniformly as `Hint`s.
//!
//! The converters are also exposed through the `dfs-convert` binary in
//! `dfs-run`, so any engine's output can be fed to any other from the shell.

#![forbid(unsafe_code)]

mod bucas;
mod csv;
mod instance;
mod site;

#[cfg(test)]
mod tests;

pub use bucas::board_to_bucas_url;
pub use csv::{load_puzzle_csv, parse_puzzle_csv};
pub use instance::{Hint, Instance, SolveOutput};
pub use site::{SiteHint, SiteInstance};

/// Errors the IO layer can surface. Kept small and printable; the harness only
/// needs a message.
#[derive(Debug)]
pub enum IoError {
    Read(std::io::Error),
    Parse(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => write!(f, "read error: {e}"),
            Self::Parse(m) => write!(f, "parse error: {m}"),
        }
    }
}

impl std::error::Error for IoError {}

impl From<std::io::Error> for IoError {
    fn from(e: std::io::Error) -> Self {
        Self::Read(e)
    }
}
