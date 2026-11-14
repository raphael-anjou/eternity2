//! Instrumented copy of A3's propagation (via A22's `Prop` with hidden singles
//! on), used only to find which cell's domain emptied in a failed
//! propagation (oracle C's wipe counts). It never drives the search; a test
//! in `main.rs gate` checks it equals `Model::propagate` on random states.

use crate::Ctx;

/// Propagate to fixpoint. Ok(()) on success, Err(Some(cell)) when `cell`'s
/// domain emptied, Err(None) when a piece lost every cell.
pub fn propagate(cx: &Ctx, d: &mut [u64], changed: &[usize]) -> Result<(), Option<usize>> {
    let m = cx.m;
    let w = m.words;
    let mut queue: Vec<usize> = changed.to_vec();
    let mut inq = vec![false; m.cells];
    for &c in changed {
        inq[c] = true;
    }
    let mut allowed = vec![0u64; w];
    let offered = |d: &[u64], c: usize, side: usize| -> u32 {
        let dc = m.dom(d, c);
        let mut mask = 0u32;
        for col in 0..m.num_colors {
            if dc
                .iter()
                .zip(&cx.side_mask[side][col])
                .any(|(a, b)| a & b != 0)
            {
                mask |= 1 << col;
            }
        }
        mask
    };
    loop {
        while let Some(b) = queue.pop() {
            inq[b] = false;
            for db in 0..4 {
                let Some(a) = m.neighbour[b][db] else {
                    continue;
                };
                let da = (db + 2) & 3;
                let mask = offered(d, b, db);
                allowed.iter_mut().for_each(|x| *x = 0);
                for col in 0..m.num_colors {
                    if mask & (1 << col) != 0 {
                        for (x, y) in allowed.iter_mut().zip(&cx.side_mask[da][col]) {
                            *x |= *y;
                        }
                    }
                }
                let mut changed_a = false;
                let mut empty = true;
                for k in 0..w {
                    let old = d[a * w + k];
                    let new = old & allowed[k];
                    if new != old {
                        changed_a = true;
                        d[a * w + k] = new;
                    }
                    if new != 0 {
                        empty = false;
                    }
                }
                if empty {
                    return Err(Some(a));
                }
                if changed_a && !inq[a] {
                    inq[a] = true;
                    queue.push(a);
                }
            }
        }
        let pieces = m.piece_range.len();
        let mut cnt = vec![0u8; pieces];
        let mut where_ = vec![0usize; pieces];
        let mut single: Vec<Option<usize>> = vec![None; m.cells];
        for c in 0..m.cells {
            let mut first: Option<usize> = None;
            let mut multi = false;
            for (pi, &(s, t)) in m.piece_range.iter().enumerate() {
                let pm = &cx.piece_mask[pi];
                let has = ((s >> 6)..=((t - 1) >> 6)).any(|k| d[c * w + k] & pm[k] != 0);
                if has {
                    if cnt[pi] < 2 {
                        cnt[pi] += 1;
                    }
                    where_[pi] = c;
                    match first {
                        None => first = Some(pi),
                        Some(_) => multi = true,
                    }
                }
            }
            if !multi {
                single[c] = first;
            }
        }
        let mut any = false;
        if cnt.contains(&0) {
            return Err(None);
        }
        for (c, sc) in single.iter().enumerate() {
            if let Some(pi) = *sc {
                let (s, t) = m.piece_range[pi];
                let pm = &cx.piece_mask[pi];
                for c2 in 0..m.cells {
                    if c2 == c {
                        continue;
                    }
                    let mut ch = false;
                    for k in (s >> 6)..=((t - 1) >> 6) {
                        if d[c2 * w + k] & pm[k] != 0 {
                            d[c2 * w + k] &= !pm[k];
                            ch = true;
                        }
                    }
                    if ch {
                        if m.dom(d, c2).iter().all(|&x| x == 0) {
                            return Err(Some(c2));
                        }
                        any = true;
                        if !inq[c2] {
                            inq[c2] = true;
                            queue.push(c2);
                        }
                    }
                }
            }
        }
        for pi in 0..pieces {
            if cnt[pi] == 1 {
                let c = where_[pi];
                let pm = &cx.piece_mask[pi];
                let mut ch = false;
                for k in 0..w {
                    let old = d[c * w + k];
                    let new = old & pm[k];
                    if new != old {
                        d[c * w + k] = new;
                        ch = true;
                    }
                }
                if ch {
                    if m.dom(d, c).iter().all(|&x| x == 0) {
                        return Err(Some(c));
                    }
                    any = true;
                    if !inq[c] {
                        inq[c] = true;
                        queue.push(c);
                    }
                }
            }
        }
        if !any {
            return Ok(());
        }
    }
}
