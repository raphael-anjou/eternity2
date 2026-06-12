//! Core types. Edge order is always URDL (up, right, down, left), matching
//! the e2.bucas.name URL encoding. Color 0 is the grey outer border; interior
//! colors are 1..=22 for the official set ('a' + color = bucas letter).

use serde::{Deserialize, Serialize};

pub type Color = u8;
pub const BORDER: Color = 0;

/// Rotation r = clockwise quarter-turns. Rotating URDL edges clockwise by r:
/// the edge that faced up now faces right (r=1), so new[i] = old[(i+4-r)%4].
#[inline]
pub fn rotated(e: [Color; 4], r: u8) -> [Color; 4] {
    let r = (r & 3) as usize;
    [
        e[(4 - r) & 3],
        e[(5 - r) & 3],
        e[(6 - r) & 3],
        e[(7 - r) & 3],
    ]
}

/// Lexicographically minimal cyclic rotation: the canonical form used by
/// bucas to identify a piece independent of rotation.
pub fn canonical(e: [Color; 4]) -> [Color; 4] {
    let mut best = e;
    for r in 1..4u8 {
        let c = rotated(e, r);
        if c < best {
            best = c;
        }
    }
    best
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hint {
    pub pos: u16,
    pub piece: u16,
    pub rot: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle {
    pub name: String,
    pub width: u8,
    pub height: u8,
    /// Number of interior colors (border color 0 excluded).
    pub num_colors: u8,
    /// Piece edges in rotation 0, URDL.
    pub pieces: Vec<[Color; 4]>,
    pub hints: Vec<Hint>,
}

impl Puzzle {
    pub fn cell_count(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Max achievable matched-edge score: every adjacent pair of cells plus
    /// nothing for the rim. 2wh - w - h (480 for 16x16).
    pub fn max_score(&self) -> u32 {
        let (w, h) = (self.width as u32, self.height as u32);
        2 * w * h - w - h
    }
}
