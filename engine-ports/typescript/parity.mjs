// Golden parity harness for the pure-TypeScript engine port (ALGORITHM.md §10).
//
// Reads ../lua/golden.txt (the reference outputs captured
// from the Rust engine) and asserts every GEN / OFF / OFFHINT / PATH / SOLVE /
// OFFICIALRUN record against this TS port — field for field, count for count.
// Matching the node/attempt/backtrack counts proves we walk the identical
// search tree, not merely that we both solve.
//
// HOW TO RUN (Node 22, ESM, zero new deps):
//
//   node engine-ports/typescript/parity.mjs
//
// Node 22.0 lacks `--experimental-strip-types`, so this harness transpiles the
// .ts engine modules to a temp dir using the `typescript` already in the
// project's devDependencies (no extra install), rewriting the `@/lib/types`
// alias and `.ts` import specifiers to plain relative `.js`, then imports the
// compiled output. The only inputs are golden.txt and the .ts sources.
//
// Exits non-zero on any failure.

import { readFileSync, writeFileSync, mkdtempSync, mkdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { createRequire } from "node:module";

const __dirname = dirname(fileURLToPath(import.meta.url));
// `typescript` lives in the web workspace's node_modules (this port has no deps
// of its own), so resolve it from there regardless of the current directory.
const require = createRequire(join(__dirname, "..", "..", "web", "package.json"));
const ts = require("typescript");
const repoRoot = join(__dirname, "..", ".."); // repo root: engine-ports/typescript -> up 2
// The lua port carries the fullest golden capture (GEN/OFF/PATH/SOLVE); the TS
// port validates against it, as it did before the ports were gathered here.
const goldenPath = join(__dirname, "..", "lua", "golden.txt");
// The shared domain types live in the web app; the port reuses them.
const libTypesPath = join(repoRoot, "web", "src", "lib", "types.ts");

// --- Transpile the TS port to a temp dir ------------------------------------
// We compile: src/lib/types.ts, plus all engine-ts/*.ts. The `@/lib/types`
// alias becomes a relative import; `.ts` specifiers become `.js`.

const outDir = mkdtempSync(join(tmpdir(), "e2-parity-"));
const libOut = join(outDir, "lib");
const engineOut = join(outDir, "engine-ts");
mkdirSync(libOut, { recursive: true });
mkdirSync(engineOut, { recursive: true });
// Mark the temp tree as ESM so the emitted .js files load as modules.
writeFileSync(join(outDir, "package.json"), JSON.stringify({ type: "module" }));

function transpile(srcPath, rewrites) {
  let source = readFileSync(srcPath, "utf8");
  for (const [from, to] of rewrites) {
    source = source.split(from).join(to);
  }
  const result = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ESNext,
      target: ts.ScriptTarget.ES2023,
      verbatimModuleSyntax: false,
    },
  });
  return result.outputText;
}

// src/lib/types.ts -> outDir/lib/types.js (no special imports).
writeFileSync(join(libOut, "types.js"), transpile(libTypesPath, []));

// engine-ts modules: rewrite "@/lib/types" -> "../lib/types.js" and ".ts" -> ".js".
const engineFiles = [
  "official_data.ts",
  "types.ts",
  "rng.ts",
  "generator.ts",
  "official.ts",
  "paths.ts",
  "solver.ts",
  "index.ts",
];
for (const f of engineFiles) {
  const js = transpile(join(__dirname, f), [
    ['"@/lib/types"', '"../lib/types.js"'],
    ['.ts"', '.js"'],
  ]);
  writeFileSync(join(engineOut, f.replace(/\.ts$/, ".js")), js);
}

const engine = await import(pathToFileURL(join(engineOut, "index.js")).href);
const {
  getGeneratedPuzzle,
  getGeneratedPuzzleFramed,
  getOfficialPuzzle,
  getPath,
  getPathKinds,
  getBoardScore,
  createSolver,
} = engine;

// --- Test runner ------------------------------------------------------------
let pass = 0;
let fail = 0;
function check(cond, msg) {
  if (cond) {
    pass += 1;
  } else {
    fail += 1;
    console.log("  FAIL: " + msg);
  }
}

const goldenLines = readFileSync(goldenPath, "utf8")
  .split("\n")
  .filter((l) => l.trim().length > 0);

const split = (s) => s.trim().split(/\s+/);
const field = (tok) => Number(tok.replace(/^.*?=/, ""));
const fieldStr = (tok) => tok.replace(/^.*?=/, "");

// == generator parity ==
console.log("== generator parity ==");
for (const line of goldenLines) {
  const t = split(line);
  if (t[0] !== "GEN") continue;
  const size = Number(t[1]);
  const colors = Number(t[2]);
  const seed = Number(t[3]);
  const p = getGeneratedPuzzle(size, colors, seed);
  const got = p.pieces.map((e) => `${e[0]},${e[1]},${e[2]},${e[3]}`);
  const expect = t.slice(4);
  let ok = got.length === expect.length;
  if (ok) {
    for (let i = 0; i < got.length; i++) {
      if (got[i] !== expect[i]) {
        ok = false;
        break;
      }
    }
  }
  check(ok, `generate(${size},${colors},${seed}) pieces match`);
}

// == framed generator parity ==
console.log("== framed generator parity ==");
for (const line of goldenLines) {
  const t = split(line);
  if (t[0] !== "FRAMEDGEN") continue;
  const size = Number(t[1]);
  const colors = Number(t[2]);
  const seed = Number(t[3]);
  const p = getGeneratedPuzzleFramed(size, colors, seed, true);
  const got = p.pieces.map((e) => `${e[0]},${e[1]},${e[2]},${e[3]}`);
  const expect = t.slice(4);
  let ok = got.length === expect.length;
  if (ok) {
    for (let i = 0; i < got.length; i++) {
      if (got[i] !== expect[i]) {
        ok = false;
        break;
      }
    }
  }
  check(ok, `generateFramed(${size},${colors},${seed},true) pieces match`);
}

// == official set parity ==
console.log("== official set parity ==");
{
  const off = getOfficialPuzzle();
  for (const line of goldenLines) {
    const t = split(line);
    if (t[0] !== "OFF") continue;
    check(off.pieces.length === field(t[1]), "official piece count");
    check(off.numColors === field(t[2]), "official color count");
    check(off.hints.length === field(t[3]), "official hint count");
  }
  let hi = 0;
  for (const line of goldenLines) {
    const t = split(line);
    if (t[0] !== "OFFHINT") continue;
    const hnt = off.hints[hi];
    hi += 1;
    check(
      hnt !== undefined &&
        hnt.pos === Number(t[1]) &&
        hnt.piece === Number(t[2]) &&
        hnt.rot === Number(t[3]),
      `official hint ${hi}`,
    );
  }
}

// == path parity == (golden paths were built with seed=1; see spec.lua)
console.log("== path parity ==");
check(JSON.stringify(getPathKinds().slice().sort()) !== "[]", "path kinds non-empty");
for (const line of goldenLines) {
  const t = split(line);
  if (t[0] !== "PATH") continue;
  const kind = t[1];
  const w = Number(t[2]);
  const h = Number(t[3]);
  const path = getPath(kind, w, h, 1);
  const expect = t.slice(4).map(Number);
  let ok = path.length === expect.length;
  if (ok) {
    for (let i = 0; i < expect.length; i++) {
      if (path[i] !== expect[i]) {
        ok = false;
        break;
      }
    }
  }
  check(ok, `path ${kind} ${w}x${h}`);
}

// == solver parity (generated 4x4 seed 11, all paths) ==
console.log("== solver parity (generated 4x4 seed 11, all paths) ==");
for (const line of goldenLines) {
  const t = split(line);
  if (t[0] !== "SOLVE") continue;
  const kind = t[1];
  const eStatus = fieldStr(t[2]);
  const ePlaced = field(t[3]);
  const eScore = field(t[4]);
  const eNodes = field(t[5]);
  const eAttempts = field(t[6]);
  const eBacktracks = field(t[7]);

  const p = getGeneratedPuzzle(4, 4, 11);
  const path = getPath(kind, 4, 4, 0);
  const sv = createSolver(p, path, { useHints: true, shufflePieces: false, seed: 0 });
  let r = sv.report();
  do {
    r = sv.step(5_000_000);
  } while (r.status === "running");
  const sc = getBoardScore(p, sv.board());
  check(r.status === eStatus, `solve ${kind} status (${r.status} vs ${eStatus})`);
  check(r.placed === ePlaced, `solve ${kind} placed`);
  check(sc === eScore, `solve ${kind} score (${sc} vs ${eScore})`);
  check(r.nodes === eNodes, `solve ${kind} nodes (${r.nodes} vs ${eNodes})`);
  check(r.attempts === eAttempts, `solve ${kind} attempts (${r.attempts} vs ${eAttempts})`);
  check(r.backtracks === eBacktracks, `solve ${kind} backtracks (${r.backtracks} vs ${eBacktracks})`);
}

// == official partial-run parity (fixed step budget) ==
console.log("== official partial-run parity (fixed step budget) ==");
for (const line of goldenLines) {
  const t = split(line);
  if (t[0] !== "OFFICIALRUN") continue;
  const budget = field(t[1]);
  const eStatus = fieldStr(t[2]);
  const ePlaced = field(t[3]);
  const eBest = field(t[4]);
  const eNodes = field(t[5]);
  const eAttempts = field(t[6]);
  const eBacktracks = field(t[7]);

  const p = getOfficialPuzzle();
  const path = getPath("row-major", 16, 16, 0);
  const sv = createSolver(p, path, { useHints: true, shufflePieces: false, seed: 0 });
  const r = sv.step(budget);
  check(r.status === eStatus, `official status (${r.status} vs ${eStatus})`);
  check(r.placed === ePlaced, `official placed (${r.placed} vs ${ePlaced})`);
  check(r.bestPlaced === eBest, `official best (${r.bestPlaced} vs ${eBest})`);
  check(r.nodes === eNodes, `official nodes (${r.nodes} vs ${eNodes})`);
  check(r.attempts === eAttempts, `official attempts (${r.attempts} vs ${eAttempts})`);
  check(r.backtracks === eBacktracks, `official backtracks (${r.backtracks} vs ${eBacktracks})`);
}

// --- Done -------------------------------------------------------------------
rmSync(outDir, { recursive: true, force: true });
console.log(`\n${pass} passed, ${fail} failed`);
process.exit(fail === 0 ? 0 : 1);
