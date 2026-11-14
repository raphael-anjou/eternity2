#!/usr/bin/env python3
"""Instrument a COPY of the Track 10 engine (engine-copy/). The search itself is
not changed: same candidate order, same node counter. Added:
  --priv a,b,c      privileged colours of the quota (the original hardcodes 22,21,20)
  --maxbreaks K     (original: 10)
  --probe D --probeout FILE --probemax N
                    at every entry to depth D: colour census of the unused pieces
                    and of the open frontier, then, when the subtree is exhausted,
                    its node count, its deepest depth and the breaks used there.
Run once: python3 -I patch_engine.py
"""
import os, sys
H = os.path.dirname(os.path.abspath(__file__))
bw = os.path.join(H, 'engine-copy', 'src', 'bw.rs')
mn = os.path.join(H, 'engine-copy', 'src', 'main.rs')
s = open(bw).read()
if 'probe_depth' in s:
    sys.exit('already patched')

def rep(text, old, new, count=1):
    assert text.count(old) >= 1, old
    return text.replace(old, new, count)

s = rep(s, "    pub snapshots: Vec<(u64, Vec<u64>)>,\n}",
"""    pub snapshots: Vec<(u64, Vec<u64>)>,
    pub probe_depth: usize,
    pub probe_max: usize,
    pub probe_rows: Vec<String>,
    pub probe_seen: u64,
    sub_deepest: usize,
    sub_breaks: u8,
}""")
s = rep(s, "            snapshots: Vec::new(),\n        }",
"""            snapshots: Vec::new(),
            probe_depth: usize::MAX,
            probe_max: 0,
            probe_rows: Vec::new(),
            probe_seen: 0,
            sub_deepest: 0,
            sub_breaks: 0,
        }""")
s = rep(s, "    fn search(&mut self, depth: usize, break_count_at: &mut [u8; N]) {\n        if self.nodes >= self.node_cap {\n            return;\n        }",
"""    fn census(&self, depth: usize) -> String {
        // sides per colour among unused pieces; colours on the open frontier
        // (the bottom side of the last placed cell of each column, and the
        // right side of the last placed cell).
        let mut pool = vec![0u32; self.ncolors];
        for pid in 0..self.npieces {
            if !self.used[pid] {
                let rp = self.rotpieces[pid][0];
                for c in [rp.top, rp.right, rp.bottom, rp.left] {
                    pool[c as usize] += 1;
                }
            }
        }
        let mut fr = vec![0u32; self.ncolors];
        // cells not yet placed in scan order (clue cells are placed from the start)
        for d in depth..N {
            let pos = self.order[d];
            if self.board_piece[pos] != EMPTYP {
                continue;
            }
            let (a, l, r, b) = self.neighbor_colors(pos);
            for c in [a, l, r, b].into_iter().flatten() {
                fr[c as usize] += 1;
            }
        }
        let f = |v: &Vec<u32>| {
            v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
        };
        format!("\\"pool\\":[{}],\\"front\\":[{}]", f(&pool), f(&fr))
    }

    fn search(&mut self, depth: usize, break_count_at: &mut [u8; N]) {
        if depth == self.probe_depth && self.nodes < self.node_cap {
            let n0 = self.nodes;
            let (sd, sb) = (self.sub_deepest, self.sub_breaks);
            self.sub_deepest = depth;
            self.sub_breaks = self.breaks_used;
            let cen = if self.probe_rows.len() < self.probe_max {
                Some(self.census(depth))
            } else {
                None
            };
            let c0 = self.completions;
            self.search_inner(depth, break_count_at);
            self.probe_seen += 1;
            if let Some(cen) = cen {
                if self.nodes < self.node_cap {
                    self.probe_rows.push(format!(
                        "{{\\"at\\":{},\\"sub\\":{},\\"deep\\":{},\\"brk\\":{},\\"priv\\":{},\\"done\\":{},{}}}",
                        n0,
                        self.nodes - n0,
                        self.sub_deepest,
                        self.sub_breaks,
                        self.priv_on_board,
                        self.completions - c0,
                        cen
                    ));
                }
            }
            self.sub_deepest = sd.max(self.sub_deepest);
            if self.sub_deepest == sd {
                self.sub_breaks = sb;
            }
            return;
        }
        self.search_inner(depth, break_count_at);
    }

    fn search_inner(&mut self, depth: usize, break_count_at: &mut [u8; N]) {
        if self.nodes >= self.node_cap {
            return;
        }
        if depth > self.sub_deepest {
            self.sub_deepest = depth;
            self.sub_breaks = self.breaks_used;
        }""")
open(bw, 'w').write(s)

m = open(mn).read()
m = rep(m, "            let mut s = bw::Solver::new(&p, &[22, 21, 20], quota, unlock.clone(), 10, cap, cps);",
"""            let privc: Vec<u8> = m
                .get("priv")
                .map(|s| s.split(',').map(|t| t.parse().unwrap()).collect())
                .unwrap_or_else(|| vec![22, 21, 20]);
            let maxb: u8 = get(&m, "maxbreaks", 10u8);
            let mut s = bw::Solver::new(&p, &privc, quota, unlock.clone(), maxb, cap, cps);
            s.probe_depth = get(&m, "probe", usize::MAX);
            s.probe_max = get(&m, "probemax", 20000usize);
            let probeout = m.get("probeout").cloned();""")
m = rep(m, """            let secs = t0.elapsed().as_secs_f64();
            let snaps: Vec<String> = s
                .snapshots""",
"""            let secs = t0.elapsed().as_secs_f64();
            if let Some(po) = probeout {
                std::fs::write(&po, s.probe_rows.join("\\n") + "\\n").unwrap();
                eprintln!("probe: {} entries seen, {} rows written", s.probe_seen, s.probe_rows.len());
            }
            let snaps: Vec<String> = s
                .snapshots""")
open(mn, 'w').write(m)
print('patched')
