// Post-build fix for path-prefix (BASE_PATH) deploys. Runs after the whole
// `react-router build` completes (see the "build" script in package.json).
//
// The problem: with BASE_PATH=/eternity2, React Router's `basename` prerenders
// every HTML route INTO build/client/eternity2/ (…/eternity2/index.html), and
// those pages reference assets at /eternity2/assets/…. But Vite writes the
// actual asset files to the outDir ROOT (build/client/assets/), and the public/
// copy, the sitemap plugin and the research-content plugin also write to the
// root (favicon.svg, robots.txt, sitemap.xml, research/<slug>.md, …). Vite's
// `base` only rewrites the URLs baked into the HTML, not the output directory.
//
// Result without this script: the on-disk layout is split-brained — HTML under
// /eternity2/, assets and public files at the root — so behind a reverse proxy
// that passes the prefix straight through (the correct, documented setup: NO
// StripPrefix), the pages load but every asset 404s.
//
// The fix: move everything at the client root (except the prefix dir itself)
// INTO the prefix dir, so the final layout is entirely under
// build/client/eternity2/ and matches the /eternity2/… URLs the HTML
// references. The prefix now survives all the way to the files, exactly as a
// pass-through proxy expects.
//
// This runs as a standalone step (not a Vite plugin) on purpose: react-router
// build writes build/client in several passes (client bundle, SSR bundle that
// reads .vite/manifest.json, then prerender). A writeBundle/closeBundle hook
// fires between passes and would move files a later pass still needs. Running
// after the CLI exits sidesteps all of that.
//
// No-op when BASE_PATH is empty (the default domain-root deploy).

import path from "node:path";
import { readdirSync, renameSync, mkdirSync, existsSync, statSync, rmdirSync } from "node:fs";
import { fileURLToPath } from "node:url";

const base = (process.env["BASE_PATH"] ?? "").replace(/^\/+|\/+$/g, "");
if (!base) process.exit(0); // root deploy: nothing to relocate

const here = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.resolve(here, "..", "build", "client");
if (!existsSync(outDir)) {
  console.error(`relocate-under-base: ${outDir} not found — did the build run?`);
  process.exit(1);
}

// The prefix's first segment is the top-level dir the prerendered HTML already
// lives in; keep it and move everything else under it. (BASE_PATH is almost
// always a single segment like /eternity2, but /a/b keeps "a" and nests the
// rest.)
const keepTop = base.split("/")[0];
const target = path.join(outDir, base);
mkdirSync(target, { recursive: true });

let moved = 0;
for (const entry of readdirSync(outDir)) {
  if (entry === keepTop) continue; // the prefix dir itself
  moveInto(path.join(outDir, entry), path.join(target, entry));
}

console.log(`relocate-under-base: moved ${moved} root entr${moved === 1 ? "y" : "ies"} under /${base}/`);

/** Move `from` to `to`. If both are directories that already exist, merge child
 *  by child (the research/ dir is prerendered under the prefix as HTML while its
 *  .md siblings land at the root — they must merge, not clobber). On any file
 *  collision keep the existing (prefixed) copy. */
function moveInto(from, to) {
  if (existsSync(to)) {
    if (statSync(from).isDirectory() && statSync(to).isDirectory()) {
      for (const child of readdirSync(from)) {
        moveInto(path.join(from, child), path.join(to, child));
      }
      // Drop the source dir now that the merge moved its files. It may retain
      // children if a file collided (prefixed copy kept); rmdir only removes an
      // empty dir, so guard on that rather than forcing a recursive delete.
      if (readdirSync(from).length === 0) rmdirSync(from);
      return;
    }
    return; // collision on a file: keep the prefixed version, drop the root one
  }
  mkdirSync(path.dirname(to), { recursive: true });
  renameSync(from, to);
  moved++;
}
