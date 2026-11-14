//! Puzzle input: the legacy CSV of the single-core benchmark (16-bit binary
//! colour words, 65535 = border = colour 0, optional x,y,rot hint columns),
//! a fixed-cell file (one `pos piece rot` triple per line), and a seeded
//! planted-board generator for the validation gate.

use std::fs;

pub type Edges = [u8; 4]; // [top, right, bottom, left]

/// Clockwise rotation by r quarter turns; same convention as blackwood_bt.rs.
pub fn rotate(e: Edges, r: u8) -> Edges {
    let [t, ri, b, l] = e;
    match r & 3 {
        0 => [t, ri, b, l],
        1 => [l, t, ri, b],
        2 => [b, l, t, ri],
        _ => [ri, b, l, t],
    }
}

pub struct Puzzle {
    pub n: usize,
    pub pieces: Vec<Edges>,
    /// (pos, piece, rot)
    pub hints: Vec<(usize, u16, u8)>,
}

fn color(s: &str) -> u8 {
    let v = u32::from_str_radix(s.trim(), 2).expect("binary colour word");
    if v == 65535 {
        0
    } else {
        u8::try_from(v).expect("colour fits u8")
    }
}

pub fn load_csv(path: &str) -> Puzzle {
    let raw = fs::read_to_string(path).expect("read csv");
    let mut lines = raw.lines().filter(|l| !l.trim().is_empty());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut pieces = Vec::new();
    let mut hints = Vec::new();
    for line in lines {
        let c: Vec<&str> = line.split(',').collect();
        let e = [color(c[0]), color(c[1]), color(c[2]), color(c[3])];
        let id = pieces.len();
        pieces.push(e);
        if c.len() >= 7 {
            let x: usize = c[4].trim().parse().unwrap_or(0);
            let y: usize = c[5].trim().parse().unwrap_or(0);
            let r: u8 = c[6].trim().parse().unwrap_or(0);
            if !(x == 0 && y == 0 && r == 0) {
                hints.push((y * n + x, id as u16, r));
            }
        }
    }
    assert_eq!(pieces.len(), n * n);
    Puzzle { n, pieces, hints }
}

pub fn load_fixed(path: &str) -> Vec<(usize, u16, u8)> {
    let raw = fs::read_to_string(path).expect("read fixed-cell file");
    raw.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| {
            let v: Vec<usize> = l.split_whitespace().map(|t| t.parse().unwrap()).collect();
            (v[0], v[1] as u16, v[2] as u8)
        })
        .collect()
}

// splitmix-seeded xoshiro256**, identical to blackwood_bt.rs.
#[derive(Clone)]
pub struct Rng {
    s: [u64; 4],
}
impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut next = || {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            x ^ (x >> 31)
        };
        Self {
            s: [next(), next(), next(), next()],
        }
    }
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let result = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }
    #[inline]
    pub fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % (n as u64)) as usize
    }
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        let len = v.len();
        for i in (1..len).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
}

/// Planted n x n board: frame joints (between two frame cells) use colours
/// 1..=cf, every other inner joint uses cf+1..=cf+ci, border sides are 0.
/// Returns the puzzle (pieces shuffled, randomly rotated) and the planted
/// placement as (pos, piece, rot).
pub fn generate(n: usize, cf: u8, ci: u8, seed: u64) -> (Puzzle, Vec<(usize, u16, u8)>) {
    let mut rng = Rng::new(seed ^ 0x5EED_0000_0000_0000);
    let is_frame = |x: usize, y: usize| x == 0 || y == 0 || x == n - 1 || y == n - 1;
    // horizontal joint between (x,y) and (x+1,y); vertical between (x,y) and (x,y+1)
    let mut h = vec![0u8; n * n];
    let mut v = vec![0u8; n * n];
    for y in 0..n {
        for x in 0..n {
            if x + 1 < n {
                let both_frame = is_frame(x, y) && is_frame(x + 1, y) && (y == 0 || y == n - 1);
                h[y * n + x] = if both_frame {
                    1 + rng.below(cf as usize) as u8
                } else {
                    cf + 1 + rng.below(ci as usize) as u8
                };
            }
            if y + 1 < n {
                let both_frame = is_frame(x, y) && is_frame(x, y + 1) && (x == 0 || x == n - 1);
                v[y * n + x] = if both_frame {
                    1 + rng.below(cf as usize) as u8
                } else {
                    cf + 1 + rng.below(ci as usize) as u8
                };
            }
        }
    }
    let mut cells: Vec<Edges> = Vec::with_capacity(n * n);
    for y in 0..n {
        for x in 0..n {
            let t = if y == 0 { 0 } else { v[(y - 1) * n + x] };
            let b = if y == n - 1 { 0 } else { v[y * n + x] };
            let l = if x == 0 { 0 } else { h[y * n + x - 1] };
            let r = if x == n - 1 { 0 } else { h[y * n + x] };
            cells.push([t, r, b, l]);
        }
    }
    let mut perm: Vec<usize> = (0..n * n).collect();
    rng.shuffle(&mut perm);
    let mut pieces = vec![[0u8; 4]; n * n];
    let mut planted = Vec::new();
    for (pos, &pid) in perm.iter().enumerate() {
        // store the piece rotated by k; it is placed back with rotation (4-k)%4
        let k = rng.below(4) as u8;
        pieces[pid] = rotate(cells[pos], k);
        planted.push((pos, pid as u16, (4 - k) % 4));
    }
    for &(pos, pid, r) in &planted {
        assert_eq!(rotate(pieces[pid as usize], r), cells[pos]);
    }
    (
        Puzzle {
            n,
            pieces,
            hints: Vec::new(),
        },
        planted,
    )
}
