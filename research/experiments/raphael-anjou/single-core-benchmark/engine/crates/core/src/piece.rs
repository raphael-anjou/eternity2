use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type Color = u8;

// Reserved color id for the gray border ("outside") edge.
pub const BORDER: Color = 0;

pub type PieceId = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rotation(u8);

impl Rotation {
    pub const R0: Self = Self(0);
    pub const R90: Self = Self(1);
    pub const R180: Self = Self(2);
    pub const R270: Self = Self(3);

    pub const ALL: [Self; 4] = [Self::R0, Self::R90, Self::R180, Self::R270];

    #[must_use]
    pub const fn from_u8(v: u8) -> Option<Self> {
        if v < 4 { Some(Self(v)) } else { None }
    }

    #[must_use]
    pub const fn as_u8(self) -> u8 { self.0 }
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°", u32::from(self.0) * 90)
    }
}

// Edges in clockwise order starting from top: [top, right, bottom, left].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Edges([Color; 4]);

impl Edges {
    #[must_use]
    pub const fn new(top: Color, right: Color, bottom: Color, left: Color) -> Self {
        Self([top, right, bottom, left])
    }

    #[must_use]
    pub const fn top(self) -> Color { self.0[0] }
    #[must_use]
    pub const fn right(self) -> Color { self.0[1] }
    #[must_use]
    pub const fn bottom(self) -> Color { self.0[2] }
    #[must_use]
    pub const fn left(self) -> Color { self.0[3] }

    #[must_use]
    pub const fn as_array(self) -> [Color; 4] { self.0 }

    // Rotate clockwise by `r` quarter turns. R90 sends (t,r,b,l) -> (l,t,r,b)
    // i.e. each edge moves clockwise one slot.
    #[must_use]
    pub const fn rotated(self, r: Rotation) -> Self {
        let [t, ri, b, l] = self.0;
        match r.0 {
            0 => Self([t, ri, b, l]),
            1 => Self([l, t, ri, b]),
            2 => Self([b, l, t, ri]),
            _ => Self([ri, b, l, t]),
        }
    }

    // Number of unique-color positions among the 4 edges. Used to count
    // distinct rotations: a piece with 4 identical edges has 1 unique
    // orientation, a piece with one mirror axis has 2, etc.
    #[must_use]
    pub fn distinct_rotation_count(self) -> u8 {
        let r0 = self;
        let r1 = self.rotated(Rotation::R90);
        let r2 = self.rotated(Rotation::R180);
        let r3 = self.rotated(Rotation::R270);
        let mut count = 1u8;
        if r1 != r0 { count += 1; }
        if r2 != r0 && r2 != r1 { count += 1; }
        if r3 != r0 && r3 != r1 && r3 != r2 { count += 1; }
        count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Piece {
    pub id: PieceId,
    pub edges: Edges,
}

impl Piece {
    #[must_use]
    pub const fn new(id: PieceId, edges: Edges) -> Self {
        Self { id, edges }
    }

    #[must_use]
    pub const fn is_corner(self) -> bool {
        let e = self.edges.as_array();
        let mut borders = 0u8;
        let mut i = 0;
        while i < 4 {
            if e[i] == BORDER { borders += 1; }
            i += 1;
        }
        borders == 2
    }

    #[must_use]
    pub const fn is_edge(self) -> bool {
        let e = self.edges.as_array();
        let mut borders = 0u8;
        let mut i = 0;
        while i < 4 {
            if e[i] == BORDER { borders += 1; }
            i += 1;
        }
        borders == 1
    }

    #[must_use]
    pub const fn is_inner(self) -> bool {
        let e = self.edges.as_array();
        let mut i = 0;
        while i < 4 {
            if e[i] == BORDER { return false; }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_cycles() {
        let e = Edges::new(1, 2, 3, 4);
        assert_eq!(e.rotated(Rotation::R0), e);
        assert_eq!(e.rotated(Rotation::R90), Edges::new(4, 1, 2, 3));
        assert_eq!(e.rotated(Rotation::R180), Edges::new(3, 4, 1, 2));
        assert_eq!(e.rotated(Rotation::R270), Edges::new(2, 3, 4, 1));
    }

    #[test]
    fn distinct_rotation_count_works() {
        assert_eq!(Edges::new(1, 1, 1, 1).distinct_rotation_count(), 1);
        assert_eq!(Edges::new(1, 2, 1, 2).distinct_rotation_count(), 2);
        assert_eq!(Edges::new(1, 2, 3, 4).distinct_rotation_count(), 4);
    }

    #[test]
    fn corner_edge_inner_classification() {
        let corner = Piece::new(0, Edges::new(BORDER, 1, 2, BORDER));
        let edge = Piece::new(1, Edges::new(BORDER, 1, 2, 3));
        let inner = Piece::new(2, Edges::new(1, 2, 3, 4));
        assert!(corner.is_corner());
        assert!(edge.is_edge());
        assert!(inner.is_inner());
        assert!(!corner.is_edge());
        assert!(!corner.is_inner());
    }
}
