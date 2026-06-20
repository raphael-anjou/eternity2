//! The official Eternity II piece set, embedded from the canonical CSV
//! (ALGORITHM.md §5).
//!
//! CSV format: line 1 = board size; then one piece per line as
//! `top,right,bottom,left,x,y,rotation` where each color is a 16-bit binary
//! word (65535 = grey border = color 0). Pieces whose (x, y, rotation)
//! columns are not all zero are the five official clue pieces, pinned at
//! position y*16+x with the given clockwise rotation.

use crate::types::{Hint, Puzzle, BORDER};

const OFFICIAL_CSV: &str = include_str!("../data/official_eternity2.csv");

fn parse_color_word(s: &str) -> u8 {
    let v = u32::from_str_radix(s.trim(), 2).expect("invalid binary color word in official CSV");
    if v == 65535 {
        BORDER
    } else {
        u8::try_from(v).expect("color out of u8 range in official CSV")
    }
}

pub fn official_puzzle() -> Puzzle {
    let mut lines = OFFICIAL_CSV.lines().filter(|l| !l.trim().is_empty());
    let size: u8 = lines.next().unwrap().trim().parse().unwrap();

    let mut pieces = Vec::with_capacity(size as usize * size as usize);
    let mut hints = Vec::new();
    let mut max_color = 0u8;

    for (id, line) in lines.enumerate() {
        let cols: Vec<&str> = line.split(',').collect();
        let edges = [
            parse_color_word(cols[0]),
            parse_color_word(cols[1]),
            parse_color_word(cols[2]),
            parse_color_word(cols[3]),
        ];
        for &c in &edges {
            max_color = max_color.max(c);
        }
        pieces.push(edges);

        if cols.len() >= 7 {
            let x: u16 = cols[4].trim().parse().unwrap_or(0);
            let y: u16 = cols[5].trim().parse().unwrap_or(0);
            let rot: u8 = cols[6].trim().parse().unwrap_or(0);
            if x != 0 || y != 0 || rot != 0 {
                hints.push(Hint {
                    pos: y * size as u16 + x,
                    piece: id as u16,
                    rot,
                });
            }
        }
    }

    Puzzle {
        name: "official_eternity2".into(),
        width: size,
        height: size,
        num_colors: max_color,
        pieces,
        hints,
    }
}
