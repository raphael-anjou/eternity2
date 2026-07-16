//! The legacy CSV puzzle format — the one the standalone community engines
//! (McGavin's C, Blackwood's C#) read, and the sibling benchmark's
//! `variant_NN.csv`. Faithful to `eternity2-puzzle-io`:
//!
//! ```text
//! line 1:        board size N
//! lines 2..N²+1: top,right,bottom,left,x,y,rotation
//! ```
//!
//! Each edge color is a 16-bit zero-padded binary word; `1111111111111111`
//! (65535) is the gray border, mapped to interior id 0. A piece line whose
//! `(x, y, rotation)` are not all zero is a pinned hint at `pos = y*N + x`.

use dfs_core::Pieces;

use crate::{Hint, Instance, IoError};

/// Interior colors are represented as small integers; the border word 65535
/// maps to 0.
fn parse_color_word(s: &str) -> Result<u8, IoError> {
    let value = u32::from_str_radix(s.trim(), 2)
        .map_err(|_| IoError::Parse(format!("invalid 16-bit binary: {s:?}")))?;
    if value == 65535 {
        return Ok(0);
    }
    u8::try_from(value).map_err(|_| IoError::Parse(format!("color {value} doesn't fit u8")))
}

/// Parse the CSV text into an [`Instance`].
pub fn parse_puzzle_csv(name: &str, text: &str) -> Result<Instance, IoError> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let size_line = lines
        .next()
        .ok_or_else(|| IoError::Parse("missing size header".into()))?;
    let size: u32 = size_line
        .trim()
        .parse()
        .map_err(|_| IoError::Parse(format!("bad size {size_line:?}")))?;

    let mut edges: Vec<[u8; 4]> = Vec::with_capacity((size * size) as usize);
    let mut hints: Vec<Hint> = Vec::new();
    let mut max_color = 0u8;

    for (id, line) in lines.enumerate() {
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 4 {
            return Err(IoError::Parse(format!("too few columns: {line:?}")));
        }
        let e = [
            parse_color_word(cols[0])?,
            parse_color_word(cols[1])?,
            parse_color_word(cols[2])?,
            parse_color_word(cols[3])?,
        ];
        for &c in &e {
            max_color = max_color.max(c);
        }
        edges.push(e);

        if cols.len() >= 7 {
            let x: i64 = cols[4].trim().parse().unwrap_or(0);
            let y: i64 = cols[5].trim().parse().unwrap_or(0);
            let rot: i64 = cols[6].trim().parse().unwrap_or(0);
            let all_zero = x == 0 && y == 0 && rot == 0;
            if !all_zero
                && (0..i64::from(size)).contains(&x)
                && (0..i64::from(size)).contains(&y)
                && (0..=3).contains(&rot)
            {
                hints.push(Hint {
                    pos: (y as u32 * size + x as u32) as u16,
                    piece: id as u16,
                    rot: rot as u8,
                });
            }
        }
    }

    Ok(Instance {
        name: name.to_string(),
        width: size as u8,
        height: size as u8,
        num_colors: max_color,
        pieces: Pieces::new(edges),
        hints,
    })
}

/// Load and parse a CSV puzzle file.
pub fn load_puzzle_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Instance, IoError> {
    let text = std::fs::read_to_string(path.as_ref())?;
    let name = path
        .as_ref()
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("puzzle")
        .to_string();
    parse_puzzle_csv(&name, &text)
}

impl Instance {
    /// Render this instance back to the legacy CSV format, so a board or puzzle
    /// converted from site JSON can be handed to a CSV-reading community engine.
    #[must_use]
    pub fn to_csv(&self) -> String {
        // pos -> (rot) for pinned hints, so we can re-emit x,y,rotation columns.
        let mut hint_at: std::collections::HashMap<u16, (u16, u8)> = std::collections::HashMap::new();
        for h in &self.hints {
            hint_at.insert(h.piece, (h.pos, h.rot));
        }
        let mut out = format!("{}\n", self.width);
        for (id, p) in self.pieces.iter() {
            let word = |c: u8| -> String {
                if c == 0 {
                    "1111111111111111".to_string()
                } else {
                    format!("{c:016b}")
                }
            };
            let e = p.edges;
            if let Some((pos, rot)) = hint_at.get(&id) {
                let (x, y) = (pos % u16::from(self.width), pos / u16::from(self.width));
                out.push_str(&format!(
                    "{},{},{},{},{},{},{}\n",
                    word(e[0]), word(e[1]), word(e[2]), word(e[3]), x, y, rot
                ));
            } else {
                out.push_str(&format!(
                    "{},{},{},{},0,0,0\n",
                    word(e[0]), word(e[1]), word(e[2]), word(e[3])
                ));
            }
        }
        out
    }
}
