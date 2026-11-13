// Entropy density h(n) of the Eternity II interior grammar on width-n strips.
//
// Pieces reusable (the 196 interior pieces, 4 rotations each = 784 placements,
// 17 interior colours), every internal joint must match, the strip's left and
// right sides are free. lambda_n is the spectral radius of the row transfer
// operator, found by power iteration with a cell-by-cell (broken-profile)
// product; h(n) = log10(lambda_n) / n.
//
// Node, no dependencies, single thread:  node strip_entropy.js 5   (widths 1..5)
//                                         node strip_entropy.js 6 6 (width 6 only, 1.7 GB, float32)
"use strict";
const fs = require("fs");
const path = require("path");

const csv = path.join(__dirname, "..", "..", "..", "..", "..", "engine", "data", "official_eternity2.csv");
const rows = fs.readFileSync(csv, "utf8").split(/\r?\n/).filter((l) => l.trim()).slice(1);
const pieces = rows.map((l) => l.split(",").slice(0, 4).map((x) => (parseInt(x, 2) === 65535 ? 0 : parseInt(x, 2))));
const interior = pieces.filter((p) => !p.includes(0));
if (interior.length !== 196) throw new Error("expected 196 interior pieces");
const colours = [...new Set(interior.flat())].sort((a, b) => a - b);
if (colours.length !== 17) throw new Error("expected 17 interior colours");
const ci = new Map(colours.map((c, i) => [c, i]));
const C = 17;
// placements: [top, left, bottom, right]; stored order is top,right,bottom,left
const PL = [];
for (const p of interior) {
  for (let k = 0; k < 4; k++) {
    const q = [0, 1, 2, 3].map((i) => p[(i - k + 4) % 4]);
    PL.push([ci.get(q[0]), ci.get(q[3]), ci.get(q[2]), ci.get(q[1])]);
  }
}
const PT = Int32Array.from(PL.map((x) => x[1] * C + x[0])); // input index (left, top)
const PO = Int32Array.from(PL.map((x) => x[2] * C + x[3])); // output index (bottom, right)

function lambda(n, F) {
  const sN = C ** n;
  const arr = new F(sN * C);
  const u = new Float64Array(sN).fill(1);
  const inb = new Float64Array(C * C);
  const out = new Float64Array(C * C);
  let lam = 0;
  for (let it = 0; it < 400; it++) {
    // broadcast u over the free left colour (digit 0, most significant)
    for (let h = 0; h < C; h++) for (let t = 0; t < sN; t++) arr[h * sN + t] = u[t];
    for (let k = 0; k < n; k++) {
      const s = C ** (n - k - 1); // stride of digit k+1
      const S = C * s; // stride of digit k
      const blk = C * S; // span of digits k..n
      for (let pre = 0; pre < arr.length; pre += blk) {
        for (let suf = 0; suf < s; suf++) {
          const base = pre + suf;
          for (let h = 0; h < C; h++) for (let t = 0; t < C; t++) inb[h * C + t] = arr[base + h * S + t * s];
          out.fill(0);
          for (let p = 0; p < PT.length; p++) out[PO[p]] += inb[PT[p]];
          for (let b = 0; b < C; b++) for (let r = 0; r < C; r++) arr[base + b * S + r * s] = out[b * C + r];
        }
      }
    }
    // sum over the free right colour (digit n, least significant)
    let tot = 0;
    for (let t = 0; t < sN; t++) {
      let acc = 0;
      for (let r = 0; r < C; r++) acc += arr[t * C + r];
      u[t] = acc;
      tot += acc;
    }
    const prev = lam;
    lam = tot / sN; // u was normalised to mean 1
    for (let t = 0; t < sN; t++) u[t] = (u[t] / tot) * sN;
    if (it > 3 && Math.abs(lam - prev) < 1e-9 * lam) break;
  }
  return lam;
}

const hi = Number(process.argv[2] || 5);
const lo = Number(process.argv[3] || 1);
let prevLam = null;
for (let n = lo; n <= hi; n++) {
  const t0 = Date.now();
  const lam = lambda(n, n >= 6 ? Float32Array : Float64Array);
  const h = Math.log10(lam) / n;
  const inc = prevLam ? Math.log10(lam / prevLam).toFixed(4) : "";
  console.log(`n=${n} lambda=${lam.toFixed(3)} h=${h.toFixed(4)} increment=${inc} (${((Date.now() - t0) / 1000).toFixed(1)} s)`);
  prevLam = lam;
}
