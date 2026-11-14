#!/usr/bin/env python3
"""Second instrumentation of engine-copy/ (run after patch_engine.py). Search unchanged.
Adds hist_legal[d]: calls at depth d whose board is "legal so far", meaning every placed
piece shows grey exactly on the rim sides of its cell. The break path of this engine lets
a piece break that rule at the price of one break (bw.rs, eval_candidate); the first-moment
model counts legal boards only. A node is legal only if its parent is, so hist_legal is the
histogram a legal-only engine with the same order would produce.
Run once: python3 -I patch_engine2.py
"""
import os, sys
H = os.path.dirname(os.path.abspath(__file__))
bw = os.path.join(H, 'engine-copy', 'src', 'bw.rs')
mn = os.path.join(H, 'engine-copy', 'src', 'main.rs')
s = open(bw).read()
if 'hist_legal' in s:
    sys.exit('already patched')
def rep(text, old, new):
    assert text.count(old) == 1, (text.count(old), old)
    return text.replace(old, new)
s = rep(s, "    sub_deepest: usize,\n    sub_breaks: u8,\n}", "    sub_deepest: usize,\n    sub_breaks: u8,\n    illegal: u32,\n    pub hist_legal: Vec<u64>,\n}")
s = rep(s, "            sub_deepest: 0,\n            sub_breaks: 0,\n        }", "            sub_deepest: 0,\n            sub_breaks: 0,\n            illegal: 0,\n            hist_legal: vec![0; N + 1],\n        }")
s = rep(s, "        self.hist[depth] += 1;\n", "        self.hist[depth] += 1;\n        if self.illegal == 0 {\n            self.hist_legal[depth] += 1;\n        }\n")
ILL = "u32::from((rp.top == BORDER) != bm[0] || (rp.right == BORDER) != bm[1] || (rp.bottom == BORDER) != bm[2] || (rp.left == BORDER) != bm[3])"
s = rep(s, "                self.priv_on_board += u16::from(rp.heuristic_count);\n                self.search(depth + 1, break_count_at);\n                self.priv_on_board -= u16::from(rp.heuristic_count);",
        "                self.priv_on_board += u16::from(rp.heuristic_count);\n                let ill = " + ILL + ";\n                self.illegal += ill;\n                self.search(depth + 1, break_count_at);\n                self.illegal -= ill;\n                self.priv_on_board -= u16::from(rp.heuristic_count);")
s = rep(s, "            self.search(depth + 1, break_count_at);\n            for s in 0..4 {\n                if let Some(nc) = touched[s] {",
        "            let ill = " + ILL + ";\n            self.illegal += ill;\n            self.search(depth + 1, break_count_at);\n            self.illegal -= ill;\n            for s in 0..4 {\n                if let Some(nc) = touched[s] {")
open(bw, 'w').write(s)
m = open(mn).read()
m = rep(m, "\\\"hist\\\":{},\\\"checkpoints\\\":[{}]}}\",", "\\\"hist\\\":{},\\\"hist_legal\\\":{},\\\"checkpoints\\\":[{}]}}\",")
m = rep(m, "                fmt_vec(&s.hist),\n                snaps.join(\",\")", "                fmt_vec(&s.hist),\n                fmt_vec(&s.hist_legal),\n                snaps.join(\",\")")
open(mn, 'w').write(m)
print('patched')
