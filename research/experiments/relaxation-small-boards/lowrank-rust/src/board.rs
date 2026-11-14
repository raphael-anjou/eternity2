//! TOY BOARDS ONLY. Loads one board exported by py/export.py.

use std::fs;
use std::path::{Path, PathBuf};

pub const DIAG: u8 = 0;
pub const CONFLICT: u8 = 1;
pub const FREE: u8 = 2;

pub struct Board {
    pub name: String,
    pub dir: PathBuf,
    pub n: usize,
    pub p: usize,
    pub nc: usize,
    pub nt: usize,
    pub cell: Vec<usize>,
    pub tile: Vec<usize>,
    pub cols: Vec<[i64; 4]>,
    pub nb: Vec<[i64; 4]>,
    pub cells: Vec<Vec<usize>>,
    pub tiles: Vec<Vec<usize>>,
    pub conf: Vec<(usize, usize)>,
    pub nfree: usize,
    /// P x P pair kind: DIAG, CONFLICT or FREE.
    pub kind: Vec<u8>,
    /// P x P: index of the pair in `conf` for conflict pairs, u32::MAX otherwise.
    pub conf_idx: Vec<u32>,
    /// P x P masks as 1.0 / 0.0 (branch-free kernels)
    pub confm: Vec<f64>,
    pub freem: Vec<f64>,
    pub sols: Vec<Vec<usize>>,
}

fn rows(path: &Path) -> Vec<Vec<i64>> {
    let s = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    s.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.split_whitespace().map(|a| a.parse::<i64>().unwrap()).collect())
        .collect()
}

pub fn read_f64(path: &Path) -> Vec<f64> {
    let b = fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    b.chunks_exact(8).map(|c| f64::from_le_bytes(c.try_into().unwrap())).collect()
}

impl Board {
    pub fn load(dir: &Path) -> Board {
        let meta = &rows(&dir.join("meta.txt"))[0];
        let (n, p, nc, nt, k, f, ns) = (
            meta[0] as usize,
            meta[4] as usize,
            meta[5] as usize,
            meta[6] as usize,
            meta[7] as usize,
            meta[8] as usize,
            meta[9] as usize,
        );
        let place = rows(&dir.join("place.txt"));
        assert_eq!(place.len(), p);
        let cell: Vec<usize> = place.iter().map(|r| r[0] as usize).collect();
        let tile: Vec<usize> = place.iter().map(|r| r[1] as usize).collect();
        let cols: Vec<[i64; 4]> = place.iter().map(|r| [r[3], r[4], r[5], r[6]]).collect();
        let nb: Vec<[i64; 4]> = rows(&dir.join("nb.txt")).iter().map(|r| [r[0], r[1], r[2], r[3]]).collect();
        let to_us = |v: Vec<Vec<i64>>| -> Vec<Vec<usize>> {
            v.into_iter().map(|r| r.into_iter().map(|a| a as usize).collect()).collect()
        };
        let cells = to_us(rows(&dir.join("cells.txt")));
        let tiles = to_us(rows(&dir.join("tiles.txt")));
        assert_eq!(cells.len(), nc);
        assert_eq!(tiles.len(), nt);
        let conf: Vec<(usize, usize)> = rows(&dir.join("conflicts.txt"))
            .iter()
            .map(|r| (r[0] as usize, r[1] as usize))
            .collect();
        assert_eq!(conf.len(), k);
        let mut kind = vec![FREE; p * p];
        let mut conf_idx = vec![u32::MAX; p * p];
        for i in 0..p {
            kind[i * p + i] = DIAG;
        }
        for (q, &(i, j)) in conf.iter().enumerate() {
            assert!(i < j);
            kind[i * p + j] = CONFLICT;
            kind[j * p + i] = CONFLICT;
            conf_idx[i * p + j] = q as u32;
            conf_idx[j * p + i] = q as u32;
        }
        // the exported free list must be exactly the complement of the conflict list
        let free = rows(&dir.join("free.txt"));
        assert_eq!(free.len(), f);
        assert_eq!(f + k, p * (p - 1) / 2);
        for r in &free {
            let (i, j) = (r[0] as usize, r[1] as usize);
            assert!(i < j && kind[i * p + j] == FREE);
        }
        let confm: Vec<f64> = kind.iter().map(|&k| if k == CONFLICT { 1.0 } else { 0.0 }).collect();
        let freem: Vec<f64> = kind.iter().map(|&k| if k == FREE { 1.0 } else { 0.0 }).collect();
        let sols = if ns > 0 { to_us(rows(&dir.join("sols.txt"))) } else { vec![] };
        assert_eq!(sols.len(), ns);
        Board {
            name: dir.file_name().unwrap().to_string_lossy().into_owned(),
            dir: dir.to_path_buf(),
            n,
            p,
            nc,
            nt,
            cell,
            tile,
            cols,
            nb,
            cells,
            tiles,
            conf,
            nfree: f,
            kind,
            conf_idx,
            confm,
            freem,
            sols,
        }
    }

    /// Sinkhorn centre X0 (board.py: 30 rounds cell then tile, then a cell normalisation).
    pub fn x0(&self) -> Vec<f64> {
        let mut x = vec![1.0; self.p];
        let norm = |x: &mut Vec<f64>, of: &Vec<usize>, k: usize| {
            let mut s = vec![0.0; k];
            for (q, &c) in of.iter().enumerate() {
                s[c] += x[q];
            }
            for (q, &c) in of.iter().enumerate() {
                x[q] /= s[c];
            }
        };
        for _ in 0..30 {
            norm(&mut x, &self.cell, self.nc);
            norm(&mut x, &self.tile, self.nt);
        }
        norm(&mut x, &self.cell, self.nc);
        x
    }

    /// Exact validity of a full assignment (one placement per cell): every cell filled by a
    /// placement of that cell, every tile used once, every rim side grey, every inner edge matched.
    pub fn is_valid(&self, js: &[usize]) -> bool {
        if js.len() != self.nc {
            return false;
        }
        let mut used = vec![false; self.nt];
        for (x, &p) in js.iter().enumerate() {
            if p >= self.p || self.cell[p] != x || used[self.tile[p]] {
                return false;
            }
            used[self.tile[p]] = true;
        }
        for x in 0..self.nc {
            let cx = self.cols[js[x]];
            for d in 0..4 {
                let y = self.nb[x][d];
                if y < 0 {
                    if cx[d] != 0 {
                        return false;
                    }
                } else if cx[d] != self.cols[js[y as usize]][(d + 2) % 4] {
                    return false;
                }
            }
        }
        true
    }

    pub fn mismatches(&self, js: &[usize]) -> usize {
        let mut m = 0;
        for x in 0..self.nc {
            for d in [1usize, 2] {
                let y = self.nb[x][d];
                if y >= 0 && self.cols[js[x]][d] != self.cols[js[y as usize]][(d + 2) % 4] {
                    m += 1;
                }
            }
        }
        m
    }

    /// bm.greedy_round: placements by decreasing x (ties by index), taken when the cell is empty
    /// and the tile unused. Returns None if a cell stays empty.
    pub fn greedy_round(&self, x: &[f64], order: &mut Vec<usize>) -> Option<Vec<usize>> {
        order.clear();
        order.extend(0..self.p);
        order.sort_by(|&a, &b| x[b].partial_cmp(&x[a]).unwrap().then(a.cmp(&b)));
        let mut js = vec![usize::MAX; self.nc];
        let mut used = vec![false; self.nt];
        let mut left = self.nc;
        for &p in order.iter() {
            let (c, t) = (self.cell[p], self.tile[p]);
            if js[c] == usize::MAX && !used[t] {
                js[c] = p;
                used[t] = true;
                left -= 1;
                if left == 0 {
                    break;
                }
            }
        }
        if left > 0 {
            None
        } else {
            Some(js)
        }
    }
}
