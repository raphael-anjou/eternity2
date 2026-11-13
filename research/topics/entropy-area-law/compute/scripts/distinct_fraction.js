// Fraction rho of colour-valid W x L interior patches that use distinct pieces.
//
// A patch is a W-wide, L-tall block of interior cells filled with interior
// placements (196 pieces x 4 rotations, pieces reusable), every internal joint
// matched, the outer boundary free. rho = (patches using 196 distinct pieces)
// / (all colour-valid patches). Sampled exactly: completion counts are computed
// cell by cell backwards, then patches are drawn uniformly by forward sampling,
// and rho is the share of drawn patches with no repeated piece. Also reports how
// often two side-by-side cells hold the same piece.
//
// Node, no dependencies, single thread, seeded:
//   node distinct_fraction.js            (the default list below)
//   node distinct_fraction.js 3 3 1000000
//   node distinct_fraction.js --seed-offset 100   (the default list, other seeds)
"use strict";
const fs = require("fs");
const path = require("path");

const csv = path.join(__dirname, "..", "..", "..", "..", "..", "engine", "data", "official_eternity2.csv");
const rows = fs.readFileSync(csv, "utf8").split(/\r?\n/).filter((l) => l.trim()).slice(1);
const pieces = rows.map((l) => l.split(",").slice(0, 4).map((x) => (parseInt(x, 2) === 65535 ? 0 : parseInt(x, 2))));
const interior = pieces.filter((p) => !p.includes(0));
const colours = [...new Set(interior.flat())].sort((a, b) => a - b);
const ci = new Map(colours.map((c, i) => [c, i]));
const C = 17;
const FREE = 17;
const B = 18; // digit base
const PL = []; // [top, left, bottom, right, pieceId]
interior.forEach((p, id) => {
  for (let k = 0; k < 4; k++) {
    const q = [0, 1, 2, 3].map((i) => p[(i - k + 4) % 4]); // top,right,bottom,left
    PL.push([ci.get(q[0]), ci.get(q[3]), ci.get(q[2]), ci.get(q[1]), id]);
  }
});
// compatible placements for (top, left), FREE = wildcard
const compat = [];
for (let t = 0; t < B; t++)
  for (let l = 0; l < B; l++)
    compat.push(PL.map((x, i) => i).filter((i) => (t === FREE || PL[i][0] === t) && (l === FREE || PL[i][1] === l)));

function mulberry32(a) {
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function run(W, L, samples, seed) {
  const nS = B ** (W + 1); // digits: h (most significant), c_0..c_{W-1}
  const pw = [];
  for (let d = 0; d <= W; d++) pw.push(B ** (W - d)); // stride of digit d (0 = h)
  const K = W * L;
  const f = [];
  for (let k = 0; k <= K; k++) f.push(null);
  f[K] = new Float32Array(nS).fill(1);
  const digit = (s, d) => Math.floor(s / pw[d]) % B;
  const next = (s, k, p) => {
    const j = k % W;
    const cj = digit(s, j + 1);
    const h = digit(s, 0);
    let ns = s + (PL[p][2] - cj) * pw[j + 1];
    const nh = j === W - 1 ? FREE : PL[p][3];
    ns += (nh - h) * pw[0];
    return ns;
  };
  for (let k = K - 1; k >= 0; k--) {
    const j = k % W;
    const g = new Float32Array(nS);
    const fn = f[k + 1];
    let mx = 0;
    for (let s = 0; s < nS; s++) {
      const h = digit(s, 0);
      if ((j === 0) !== (h === FREE)) continue; // h is free exactly at column 0
      const t = digit(s, j + 1);
      const lst = compat[t * B + h];
      let acc = 0;
      for (let i = 0; i < lst.length; i++) acc += fn[next(s, k, lst[i])];
      g[s] = acc;
      if (acc > mx) mx = acc;
    }
    for (let s = 0; s < nS; s++) g[s] /= mx;
    f[k] = g;
  }
  const rnd = mulberry32(seed);
  let start = 0;
  for (let d = 0; d <= W; d++) start += FREE * pw[d];
  let distinct = 0;
  let adjSame = 0;
  let adjTot = 0;
  const ids = new Int32Array(K);
  for (let n = 0; n < samples; n++) {
    let s = start;
    for (let k = 0; k < K; k++) {
      const j = k % W;
      const lst = compat[digit(s, j + 1) * B + digit(s, 0)];
      const fn = f[k + 1];
      let tot = 0;
      for (let i = 0; i < lst.length; i++) tot += fn[next(s, k, lst[i])];
      let r = rnd() * tot;
      let pick = lst[lst.length - 1];
      for (let i = 0; i < lst.length; i++) {
        r -= fn[next(s, k, lst[i])];
        if (r < 0) {
          pick = lst[i];
          break;
        }
      }
      ids[k] = PL[pick][4];
      s = next(s, k, pick);
    }
    const seen = new Set(ids);
    if (seen.size === K) distinct++;
    for (let k = 0; k < K; k++) {
      if (k % W !== W - 1) {
        adjTot++;
        if (ids[k] === ids[k + 1]) adjSame++;
      }
      if (k + W < K) {
        adjTot++;
        if (ids[k] === ids[k + W]) adjSame++;
      }
    }
  }
  const rho = distinct / samples;
  const se = Math.sqrt((rho * (1 - rho)) / samples);
  console.log(
    `${W}x${L} (${K} cells): rho = ${rho.toPrecision(4)} (s.e. ${se.toPrecision(2)}), ${samples} samples; ` +
      `side-by-side cells holding the same piece: ${((100 * adjSame) / adjTot).toFixed(2)}%`,
  );
}

if (process.argv.length > 2 && process.argv[2] !== "--seed-offset") {
  run(Number(process.argv[2]), Number(process.argv[3]), Number(process.argv[4]), 7);
} else {
  // --seed-offset K adds K to every seed below (K = 0 gives distinct_fraction.out).
  const off = process.argv[2] === "--seed-offset" ? Number(process.argv[3]) : 0;
  run(2, 2, 1000000, off + 1); // exact 0.892166
  run(3, 3, 1000000, off + 2); // exact 0.647680
  run(4, 4, 400000, off + 3);
  run(4, 6, 400000, off + 4);
  run(4, 8, 400000, off + 5);
  run(4, 12, 1000000, off + 6);
}
