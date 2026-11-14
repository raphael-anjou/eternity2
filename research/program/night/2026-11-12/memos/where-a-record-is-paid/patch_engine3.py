#!/usr/bin/env python3
"""Third instrumentation of engine-copy/ (after patch_engine.py and patch_engine2.py).
Adds done_legal[b]: completed boards that are legal (grey exactly on the rim) by number
of breaks b, and done_all[b] for every completed board. Search unchanged.
Run once: python3 -I patch_engine3.py
"""
import os, sys
H = os.path.dirname(os.path.abspath(__file__))
bw = os.path.join(H, 'engine-copy', 'src', 'bw.rs')
mn = os.path.join(H, 'engine-copy', 'src', 'main.rs')
s = open(bw).read()
if 'done_legal' in s:
    sys.exit('already patched')
def rep(text, old, new):
    assert text.count(old) == 1, (text.count(old), old)
    return text.replace(old, new)
s = rep(s, "    pub hist_legal: Vec<u64>,\n}", "    pub hist_legal: Vec<u64>,\n    pub done_legal: Vec<u64>,\n    pub done_all: Vec<u64>,\n}")
s = rep(s, "            hist_legal: vec![0; N + 1],\n        }", "            hist_legal: vec![0; N + 1],\n            done_legal: vec![0; 64],\n            done_all: vec![0; 64],\n        }")
s = rep(s, "            self.completions += 1;\n", "            self.completions += 1;\n            self.done_all[self.breaks_used as usize] += 1;\n            if self.illegal == 0 {\n                self.done_legal[self.breaks_used as usize] += 1;\n            }\n")
open(bw, 'w').write(s)
m = open(mn).read()
m = rep(m, "\\\"hist_legal\\\":{},", "\\\"hist_legal\\\":{},\\\"done_legal\\\":{},\\\"done_all\\\":{},")
m = rep(m, "                fmt_vec(&s.hist_legal),\n", "                fmt_vec(&s.hist_legal),\n                fmt_vec(&s.done_legal),\n                fmt_vec(&s.done_all),\n")
open(mn, 'w').write(m)
print('patched')
