#!/usr/bin/env python3
"""Fourth instrumentation of engine-copy/ (after patches 1 to 3). Search unchanged.
Probe rows gain: subl = legal nodes in the subtree below the arrival, deepl = deepest
legal depth below it (legal = grey exactly on the rim for every placed piece).
Run once: python3 -I patch_engine4.py
"""
import os, sys
H = os.path.dirname(os.path.abspath(__file__))
bw = os.path.join(H, 'engine-copy', 'src', 'bw.rs')
s = open(bw).read()
if 'nodes_legal' in s:
    sys.exit('already patched')
def rep(text, old, new):
    assert text.count(old) == 1, (text.count(old), old)
    return text.replace(old, new)
s = rep(s, "    pub done_all: Vec<u64>,\n}", "    pub done_all: Vec<u64>,\n    nodes_legal: u64,\n    sub_deepest_legal: usize,\n}")
s = rep(s, "            done_all: vec![0; 64],\n        }", "            done_all: vec![0; 64],\n            nodes_legal: 0,\n            sub_deepest_legal: 0,\n        }")
s = rep(s, "        if self.illegal == 0 {\n            self.hist_legal[depth] += 1;\n        }\n",
        "        if self.illegal == 0 {\n            self.hist_legal[depth] += 1;\n            self.nodes_legal += 1;\n            if depth > self.sub_deepest_legal {\n                self.sub_deepest_legal = depth;\n            }\n        }\n")
s = rep(s, "            let c0 = self.completions;\n            self.search_inner(depth, break_count_at);",
        "            let c0 = self.completions;\n            let nl0 = self.nodes_legal;\n            self.sub_deepest_legal = 0;\n            self.search_inner(depth, break_count_at);")
s = rep(s, "\\\"done\\\":{},{}}}\",", "\\\"done\\\":{},\\\"subl\\\":{},\\\"deepl\\\":{},{}}}\",")
s = rep(s, "                        self.completions - c0,\n                        cen", "                        self.completions - c0,\n                        self.nodes_legal - nl0,\n                        self.sub_deepest_legal,\n                        cen")
open(bw, 'w').write(s)
print('patched')
