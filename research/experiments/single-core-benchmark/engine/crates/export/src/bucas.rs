use std::collections::BTreeMap;
use std::fmt;

use eternity2_core::{Board, PieceId, Puzzle, Rotation};

/// A Bucas URL decoded against a concrete piece catalog. Unlike the viewer's
/// visual decoder, this representation proves that every non-empty edge quad
/// maps to one globally unique physical piece and rotation.
#[derive(Debug)]
pub struct BucasDecode {
    pub board: Board,
    pub puzzle_name: Option<String>,
    pub motif_order: Option<String>,
    pub used_piece_numbers: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BucasDecodeError {
    MissingBoardEdges,
    DuplicateParameter(String),
    BadPercentEncoding,
    BadDimension(String),
    DimensionMismatch { expected: (u32, u32), got: (u32, u32) },
    BadEdgeLength { expected: usize, got: usize },
    BadEdgeLetter { offset: usize, byte: u8 },
    UnknownMotifOrder(String),
    ColorOutOfRange { offset: usize, color: u8, color_count: u32 },
    BadPieceStringLength { expected: usize, got: usize },
    BadPieceNumber { position: usize, value: String },
    PiecePresenceMismatch { position: usize },
    DuplicatePieceNumber { position: usize, piece_id: PieceId },
    PieceEdgesMismatch { position: usize, piece_id: PieceId },
    NoMatchingPiece { position: usize },
    NoUniquePieceAssignment,
    AmbiguousPieceAssignment { position: usize },
    AmbiguousRotation { position: usize, piece_id: PieceId },
}

impl fmt::Display for BucasDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingBoardEdges => f.write_str("no board_edges parameter found"),
            Self::DuplicateParameter(k) => write!(f, "duplicate URL parameter {k}"),
            Self::BadPercentEncoding => f.write_str("invalid percent-encoding in URL parameters"),
            Self::BadDimension(v) => write!(f, "invalid board dimension {v:?}"),
            Self::DimensionMismatch { expected, got } => {
                write!(f, "board dimensions {got:?} do not match puzzle {expected:?}")
            }
            Self::BadEdgeLength { expected, got } => {
                write!(f, "board_edges has {got} bytes; expected exactly {expected}")
            }
            Self::BadEdgeLetter { offset, byte } => {
                write!(f, "invalid board_edges byte {byte:#x} at offset {offset}")
            }
            Self::UnknownMotifOrder(v) => write!(f, "unsupported motifs_order {v:?}"),
            Self::ColorOutOfRange { offset, color, color_count } => write!(
                f,
                "decoded color {color} at edge offset {offset} is outside 0..{color_count}"
            ),
            Self::BadPieceStringLength { expected, got } => {
                write!(f, "board_pieces has {got} bytes; expected exactly {expected}")
            }
            Self::BadPieceNumber { position, value } => {
                write!(f, "invalid board_pieces value {value:?} at cell {position}")
            }
            Self::PiecePresenceMismatch { position } => {
                write!(f, "board_edges and board_pieces disagree on emptiness at cell {position}")
            }
            Self::DuplicatePieceNumber { position, piece_id } => {
                write!(f, "piece {piece_id} is duplicated at cell {position}")
            }
            Self::PieceEdgesMismatch { position, piece_id } => write!(
                f,
                "piece {piece_id} has no rotation matching board_edges at cell {position}"
            ),
            Self::NoMatchingPiece { position } => {
                write!(f, "no catalog piece matches board_edges at cell {position}")
            }
            Self::NoUniquePieceAssignment => {
                f.write_str("edge quads cannot be assigned to distinct catalog pieces")
            }
            Self::AmbiguousPieceAssignment { position } => write!(
                f,
                "edge-only decoding has more than one valid piece assignment (witness cell {position})"
            ),
            Self::AmbiguousRotation { position, piece_id } => write!(
                f,
                "piece {piece_id} has multiple rotations matching cell {position}"
            ),
        }
    }
}

impl std::error::Error for BucasDecodeError {}

fn percent_decode(value: &str) -> Result<String, BucasDecodeError> {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'%' {
            out.push(bytes[i]);
            i += 1;
            continue;
        }
        if i + 2 >= bytes.len() {
            return Err(BucasDecodeError::BadPercentEncoding);
        }
        let hex = |b: u8| -> Option<u8> {
            match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
            }
        };
        let hi = hex(bytes[i + 1]).ok_or(BucasDecodeError::BadPercentEncoding)?;
        let lo = hex(bytes[i + 2]).ok_or(BucasDecodeError::BadPercentEncoding)?;
        out.push((hi << 4) | lo);
        i += 3;
    }
    String::from_utf8(out).map_err(|_| BucasDecodeError::BadPercentEncoding)
}

fn parse_params(input: &str, expected_edge_len: usize) -> Result<BTreeMap<String, String>, BucasDecodeError> {
    let trimmed = input.trim();
    if !trimmed.contains('=')
        && trimmed.len() == expected_edge_len
        && trimmed.bytes().all(|b| b.is_ascii_lowercase())
    {
        return Ok(BTreeMap::from([("board_edges".to_string(), trimmed.to_string())]));
    }
    let params = if let Some((_, tail)) = trimmed.split_once('#') {
        tail
    } else if let Some((_, tail)) = trimmed.split_once('?') {
        tail
    } else {
        trimmed.strip_prefix('#').unwrap_or(trimmed)
    };
    let mut out = BTreeMap::new();
    for part in params.split('&').filter(|p| !p.is_empty()) {
        let Some((raw_key, raw_value)) = part.split_once('=') else { continue };
        let key = percent_decode(raw_key)?;
        let value = percent_decode(raw_value)?;
        if out.insert(key.clone(), value).is_some() {
            return Err(BucasDecodeError::DuplicateParameter(key));
        }
    }
    Ok(out)
}

fn motif_translation(order: Option<&str>) -> Result<Option<&'static [u8]>, BucasDecodeError> {
    const JEF: &[u8] = b"atojeqlgbupkfrmhcwvsnid";
    const MARIE_BLACKWOOD: &[u8] = b"abglqejotchmrfkpudinsvw";
    match order {
        None | Some("") => Ok(None),
        Some("jef") => Ok(Some(JEF)),
        Some("marie" | "jblackwood") => Ok(Some(MARIE_BLACKWOOD)),
        Some(other) => Err(BucasDecodeError::UnknownMotifOrder(other.to_string())),
    }
}

fn matching_augment(
    cell: usize,
    candidates: &[Vec<(usize, Rotation)>],
    owners: &mut [Option<usize>],
    visited: &mut [bool],
    banned: Option<(usize, usize)>,
) -> bool {
    for &(piece, _) in &candidates[cell] {
        if banned == Some((cell, piece)) || visited[piece] {
            continue;
        }
        visited[piece] = true;
        if owners[piece].is_none_or(|other| {
            matching_augment(other, candidates, owners, visited, banned)
        }) {
            owners[piece] = Some(cell);
            return true;
        }
    }
    false
}

fn perfect_matching(
    candidates: &[Vec<(usize, Rotation)>],
    piece_count: usize,
    banned: Option<(usize, usize)>,
) -> Option<Vec<usize>> {
    let mut owners = vec![None; piece_count];
    for cell in 0..candidates.len() {
        let mut visited = vec![false; piece_count];
        if !matching_augment(cell, candidates, &mut owners, &mut visited, banned) {
            return None;
        }
    }
    let mut assigned = vec![usize::MAX; candidates.len()];
    for (piece, owner) in owners.into_iter().enumerate() {
        if let Some(cell) = owner {
            assigned[cell] = piece;
        }
    }
    assigned.iter().all(|&p| p != usize::MAX).then_some(assigned)
}

/// Decode a Bucas URL, fragment, params string, or bare `board_edges` blob.
///
/// The decoder is deliberately stricter than a viewer: lengths and motif
/// aliases are exact, explicit piece numbers are checked against the edge
/// quads, and edge-only inputs must have one globally unique piece assignment.
/// This makes the returned board suitable for uniqueness/rim/hint verification.
pub fn decode_bucas_board(puzzle: &Puzzle, input: &str) -> Result<BucasDecode, BucasDecodeError> {
    let cell_count = puzzle.cell_count() as usize;
    let params = parse_params(input, cell_count * 4)?;
    let edges_text = params
        .get("board_edges")
        .ok_or(BucasDecodeError::MissingBoardEdges)?;

    let parse_dim = |key: &str, fallback: u32| -> Result<u32, BucasDecodeError> {
        params.get(key).map_or(Ok(fallback), |v| {
            v.parse::<u32>().map_err(|_| BucasDecodeError::BadDimension(v.clone()))
        })
    };
    let square = params
        .get("puzzle_size")
        .map(|v| {
            v.parse::<u32>()
                .map_err(|_| BucasDecodeError::BadDimension(v.clone()))
        })
        .transpose()?;
    let width = square.unwrap_or(parse_dim("board_w", puzzle.width)?);
    let height = square.unwrap_or(parse_dim("board_h", puzzle.height)?);
    if (width, height) != (puzzle.width, puzzle.height) {
        return Err(BucasDecodeError::DimensionMismatch {
            expected: (puzzle.width, puzzle.height),
            got: (width, height),
        });
    }
    if edges_text.len() != cell_count * 4 {
        return Err(BucasDecodeError::BadEdgeLength {
            expected: cell_count * 4,
            got: edges_text.len(),
        });
    }

    let motif_order = params.get("motifs_order").cloned();
    let translation = motif_translation(motif_order.as_deref())?;
    let mut quads = Vec::with_capacity(cell_count);
    for (offset, chunk) in edges_text.as_bytes().chunks_exact(4).enumerate() {
        let mut q = [0u8; 4];
        for side in 0..4 {
            let byte = chunk[side];
            if !byte.is_ascii_lowercase() {
                return Err(BucasDecodeError::BadEdgeLetter {
                    offset: offset * 4 + side,
                    byte,
                });
            }
            let raw = (byte - b'a') as usize;
            let color = match translation {
                Some(map) => {
                    let Some(&mapped) = map.get(raw) else {
                        return Err(BucasDecodeError::BadEdgeLetter {
                            offset: offset * 4 + side,
                            byte,
                        });
                    };
                    mapped - b'a'
                }
                None => raw as u8,
            };
            if u32::from(color) >= puzzle.color_count {
                return Err(BucasDecodeError::ColorOutOfRange {
                    offset: offset * 4 + side,
                    color,
                    color_count: puzzle.color_count,
                });
            }
            q[side] = color;
        }
        quads.push(q);
    }

    let mut board = Board::empty(puzzle);
    let used_piece_numbers = params.contains_key("board_pieces");
    if let Some(piece_text) = params.get("board_pieces") {
        if piece_text.len() != cell_count * 3 {
            return Err(BucasDecodeError::BadPieceStringLength {
                expected: cell_count * 3,
                got: piece_text.len(),
            });
        }
        let mut used = vec![false; puzzle.pieces().len()];
        for (position, digits) in piece_text.as_bytes().chunks_exact(3).enumerate() {
            if !digits.iter().all(u8::is_ascii_digit) {
                return Err(BucasDecodeError::BadPieceNumber {
                    position,
                    value: String::from_utf8_lossy(digits).into_owned(),
                });
            }
            let ordinal = usize::from(digits[0] - b'0') * 100
                + usize::from(digits[1] - b'0') * 10
                + usize::from(digits[2] - b'0');
            let empty_edges = quads[position] == [0; 4];
            if ordinal == 0 {
                if !empty_edges {
                    return Err(BucasDecodeError::PiecePresenceMismatch { position });
                }
                continue;
            }
            if empty_edges || ordinal > puzzle.pieces().len() {
                return Err(BucasDecodeError::PiecePresenceMismatch { position });
            }
            let slot = ordinal - 1;
            let piece = &puzzle.pieces()[slot];
            if used[slot] {
                return Err(BucasDecodeError::DuplicatePieceNumber {
                    position,
                    piece_id: piece.id,
                });
            }
            let rotations: Vec<Rotation> = Rotation::ALL
                .into_iter()
                .filter(|&r| piece.edges.rotated(r).as_array() == quads[position])
                .collect();
            let Some(&rotation) = rotations.first() else {
                return Err(BucasDecodeError::PieceEdgesMismatch {
                    position,
                    piece_id: piece.id,
                });
            };
            if rotations.len() != 1 {
                return Err(BucasDecodeError::AmbiguousRotation {
                    position,
                    piece_id: piece.id,
                });
            }
            used[slot] = true;
            board.place(position as u32, piece.id, rotation);
        }
    } else {
        let positions: Vec<usize> = quads
            .iter()
            .enumerate()
            .filter_map(|(position, q)| (*q != [0; 4]).then_some(position))
            .collect();
        let mut candidates: Vec<Vec<(usize, Rotation)>> = Vec::with_capacity(positions.len());
        for &position in &positions {
            let mut cell = Vec::new();
            for (slot, piece) in puzzle.pieces().iter().enumerate() {
                let rotations: Vec<Rotation> = Rotation::ALL
                    .into_iter()
                    .filter(|&r| piece.edges.rotated(r).as_array() == quads[position])
                    .collect();
                if rotations.len() > 1 {
                    return Err(BucasDecodeError::AmbiguousRotation {
                        position,
                        piece_id: piece.id,
                    });
                }
                if let Some(&rotation) = rotations.first() {
                    cell.push((slot, rotation));
                }
            }
            if cell.is_empty() {
                return Err(BucasDecodeError::NoMatchingPiece { position });
            }
            candidates.push(cell);
        }
        let assigned = perfect_matching(&candidates, puzzle.pieces().len(), None)
            .ok_or(BucasDecodeError::NoUniquePieceAssignment)?;
        for cell in 0..assigned.len() {
            let piece_slot = assigned[cell];
            if candidates[cell].len() > 1
                && perfect_matching(
                    &candidates,
                    puzzle.pieces().len(),
                    Some((cell, piece_slot)),
                )
                .is_some()
            {
                return Err(BucasDecodeError::AmbiguousPieceAssignment {
                    position: positions[cell],
                });
            }
            let rotation = candidates[cell]
                .iter()
                .find_map(|&(slot, rotation)| (slot == piece_slot).then_some(rotation))
                .expect("matching edge must retain its rotation");
            board.place(
                positions[cell] as u32,
                puzzle.pieces()[piece_slot].id,
                rotation,
            );
        }
    }

    Ok(BucasDecode {
        board,
        puzzle_name: params.get("puzzle").cloned(),
        motif_order,
        used_piece_numbers,
    })
}

// Bucas viewer at e2.bucas.name encodes each tile as 4 letters (top, right,
// bottom, left). Color 0 (BORDER) maps to 'a'; inner colors 1..=22 map to
// 'b'..='w'. Empty cells encode as "aaaa" — same as a fully grey border tile.
fn color_to_bucas(c: u8) -> char {
    if c as usize > 22 {
        return 'a';
    }
    (b'a' + c) as char
}

#[must_use]
pub fn board_to_bucas_edges(puzzle: &Puzzle, board: &Board) -> String {
    let mut s = String::with_capacity((puzzle.cell_count() as usize) * 4);
    for pos in 0..puzzle.cell_count() {
        match board.get(pos) {
            Some((pid, rot)) => {
                if let Some(piece) = puzzle.piece(pid) {
                    let e = piece.edges.rotated(rot).as_array();
                    s.push(color_to_bucas(e[0]));
                    s.push(color_to_bucas(e[1]));
                    s.push(color_to_bucas(e[2]));
                    s.push(color_to_bucas(e[3]));
                } else {
                    s.push_str("aaaa");
                }
            }
            None => s.push_str("aaaa"),
        }
    }
    s
}

#[must_use]
pub fn bucas_url(puzzle: &Puzzle, board: &Board, puzzle_name: &str) -> String {
    let edges = board_to_bucas_edges(puzzle, board);
    format!(
        "https://e2.bucas.name/#puzzle={}&board_w={}&board_h={}&board_edges={}",
        puzzle_name, puzzle.width, puzzle.height, edges
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use eternity2_core::{Edges, Piece};

    fn puzzle() -> Puzzle {
        Puzzle::new(
            2,
            2,
            5,
            vec![
                Piece::new(0, Edges::new(0, 1, 2, 0)),
                Piece::new(1, Edges::new(0, 0, 3, 1)),
                Piece::new(2, Edges::new(2, 4, 0, 0)),
                Piece::new(3, Edges::new(3, 0, 0, 4)),
            ],
        )
        .unwrap()
    }

    fn solved_board(puzzle: &Puzzle) -> Board {
        let mut board = Board::empty(puzzle);
        for position in 0..4 {
            board.place(position, position as PieceId, Rotation::R0);
        }
        board
    }

    #[test]
    fn canonical_round_trip_without_piece_numbers() {
        let puzzle = puzzle();
        let board = solved_board(&puzzle);
        let url = bucas_url(&puzzle, &board, "test");
        let decoded = decode_bucas_board(&puzzle, &url).unwrap();
        assert_eq!(decoded.board, board);
        assert!(!decoded.used_piece_numbers);
    }

    #[test]
    fn honors_piece_numbers_and_rejects_duplicates() {
        let puzzle = puzzle();
        let edges = board_to_bucas_edges(&puzzle, &solved_board(&puzzle));
        let ok = format!("board_w=2&board_h=2&board_edges={edges}&board_pieces=001002003004");
        assert_eq!(decode_bucas_board(&puzzle, &ok).unwrap().board, solved_board(&puzzle));

        let duplicate = format!("board_w=2&board_h=2&board_edges={edges}&board_pieces=001001003004");
        assert!(matches!(
            decode_bucas_board(&puzzle, &duplicate),
            Err(BucasDecodeError::DuplicatePieceNumber { .. })
                | Err(BucasDecodeError::PieceEdgesMismatch { .. })
        ));
    }

    #[test]
    fn decodes_blackwood_motif_order() {
        const MAP: &[u8] = b"abglqejotchmrfkpudinsvw";
        let puzzle = puzzle();
        let canonical = board_to_bucas_edges(&puzzle, &solved_board(&puzzle));
        let encoded: String = canonical
            .bytes()
            .map(|canonical_byte| {
                let raw = MAP
                    .iter()
                    .position(|&mapped| mapped == canonical_byte)
                    .unwrap();
                (b'a' + raw as u8) as char
            })
            .collect();
        let input = format!(
            "board_w=2&board_h=2&motifs_order=jblackwood&board_edges={encoded}"
        );
        assert_eq!(
            decode_bucas_board(&puzzle, &input).unwrap().board,
            solved_board(&puzzle)
        );
    }

    #[test]
    fn requires_exact_edge_length() {
        let err = decode_bucas_board(&puzzle(), "board_w=2&board_h=2&board_edges=aaaa")
            .unwrap_err();
        assert!(matches!(err, BucasDecodeError::BadEdgeLength { .. }));
    }

    #[test]
    fn rejects_greedy_piece_reuse() {
        let puzzle = puzzle();
        let board = solved_board(&puzzle);
        let mut edges = board_to_bucas_edges(&puzzle, &board).into_bytes();
        let duplicated = edges[0..4].to_vec();
        edges[4..8].copy_from_slice(&duplicated);
        let input = format!(
            "board_w=2&board_h=2&board_edges={}",
            String::from_utf8(edges).unwrap()
        );
        assert!(matches!(
            decode_bucas_board(&puzzle, &input),
            Err(BucasDecodeError::NoUniquePieceAssignment)
        ));
    }
}
