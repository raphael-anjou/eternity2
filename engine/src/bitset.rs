//! Dense bitset over piece ids, shared by the strict and break-tolerant
//! solvers: "is piece p still available?" is one bit.
//!
//! This is the standard fast-solver representation (and what the website's
//! binary demo describes): a `Vec<u64>` of words, tested and flipped with
//! single bit operations instead of byte-per-piece booleans.

#[derive(Clone)]
pub(crate) struct BitSet {
    words: Vec<u64>,
}

impl BitSet {
    pub(crate) fn new(n: usize) -> Self {
        Self {
            words: vec![0u64; n.div_ceil(64)],
        }
    }

    #[inline]
    pub(crate) fn contains(&self, i: usize) -> bool {
        (self.words[i >> 6] >> (i & 63)) & 1 == 1
    }

    #[inline]
    pub(crate) fn insert(&mut self, i: usize) {
        self.words[i >> 6] |= 1u64 << (i & 63);
    }

    #[inline]
    pub(crate) fn remove(&mut self, i: usize) {
        self.words[i >> 6] &= !(1u64 << (i & 63));
    }
}
