//! The site-schema JSON (`SiteInstance`) — the format the sibling benchmark's
//! `variant_NN.json` files are written in, byte-compatible with the site's
//! `Puzzle` type. Reading it directly is what lets the DFS study run the *same*
//! ten corner-pinned instances the single-core benchmark does.

use e2_core::Pieces;
use serde::{Deserialize, Serialize};

use crate::{Hint, Instance, IoError};

/// A pinned placement in the site schema.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteHint {
    pub pos: u16,
    pub piece: u16,
    pub rot: u8,
}

/// A puzzle instance in the site schema. Field renames match the site's serde
/// derive so a file round-trips through the site's `Puzzle` unchanged.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteInstance {
    pub name: String,
    pub width: u8,
    pub height: u8,
    pub num_colors: u8,
    /// Piece edges at rotation 0, URDL, indexed by piece id.
    pub pieces: Vec<[u8; 4]>,
    pub hints: Vec<SiteHint>,
}

impl SiteInstance {
    /// Number of cells on the board (`width * height`).
    #[must_use]
    pub fn cell_count(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Maximum matched-edge score for a square board: `2wh - w - h`
    /// (480 for the official 16×16).
    #[must_use]
    pub fn max_score(&self) -> u32 {
        let (w, h) = (u32::from(self.width), u32::from(self.height));
        2 * w * h - w - h
    }
}

impl From<SiteInstance> for Instance {
    fn from(s: SiteInstance) -> Self {
        Instance {
            name: s.name,
            width: s.width,
            height: s.height,
            num_colors: s.num_colors,
            pieces: Pieces::new(s.pieces),
            hints: s
                .hints
                .into_iter()
                .map(|h| Hint {
                    pos: h.pos,
                    piece: h.piece,
                    rot: h.rot,
                })
                .collect(),
        }
    }
}

impl Instance {
    /// Read a site-schema variant JSON straight into an [`Instance`].
    pub fn from_site_json<P: AsRef<std::path::Path>>(path: P) -> Result<Self, IoError> {
        let text = std::fs::read_to_string(path.as_ref())?;
        let site: SiteInstance =
            serde_json::from_str(&text).map_err(|e| IoError::Parse(e.to_string()))?;
        Ok(site.into())
    }

    /// Render this instance back to the site schema, for the reverse conversion
    /// (e.g. a CSV puzzle re-emitted as site JSON for a native engine).
    #[must_use]
    pub fn to_site(&self) -> SiteInstance {
        SiteInstance {
            name: self.name.clone(),
            width: self.width,
            height: self.height,
            num_colors: self.num_colors,
            pieces: self.pieces.iter().map(|(_, p)| p.edges).collect(),
            hints: self
                .hints
                .iter()
                .map(|h| SiteHint {
                    pos: h.pos,
                    piece: h.piece,
                    rot: h.rot,
                })
                .collect(),
        }
    }
}
