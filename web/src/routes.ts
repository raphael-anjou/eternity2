import { type RouteConfig, index, route, layout } from "@react-router/dev/routes";

// One set of page modules, mounted twice: at the root (English) and under /fr
// (French). We deliberately avoid a per-language `basename` — basename +
// prerender + ssr:false is broken in react-router since 7.2.0
// (remix-run/react-router#13615). The active language is derived from the URL
// inside the shared layout, so the same module files back both language trees
// (no content duplication — only the dictionaries inside each page switch).
//
// React Router derives a route id from the module path, which would collide
// when a module is used twice, so each entry gets an explicit unique id.
const PAGES = [
  { path: "", file: "pages/Home.tsx", id: "home" },
  { path: "puzzle", file: "pages/Puzzle.tsx", id: "puzzle" },
  { path: "playground", file: "pages/playground/Hub.tsx", id: "playground" },
  { path: "playground/watch", file: "pages/playground/Watch.tsx", id: "watch" },
  { path: "playground/solve", file: "pages/playground/Solve.tsx", id: "solve" },
  { path: "playground/paths", file: "pages/playground/Paths.tsx", id: "paths" },
  { path: "playground/print", file: "pages/playground/Print.tsx", id: "print" },
  { path: "algorithms", file: "pages/Algorithms.tsx", id: "algorithms" },
  { path: "research", file: "pages/Research.tsx", id: "research" },
  { path: "research/why", file: "pages/research/why/Hub.tsx", id: "why" },
  { path: "research/why/walls-and-methods", file: "pages/research/why/walls-and-methods.tsx", id: "walls-and-methods" },
  { path: "research/why/mismatch-geometry", file: "pages/research/why/mismatch-geometry.tsx", id: "mismatch-geometry" },
  { path: "research/why/prune-vs-speed", file: "pages/research/why/prune-vs-speed.tsx", id: "prune-vs-speed" },
  { path: "research/why/complex-theory", file: "pages/research/why/complex-theory.tsx", id: "complex-theory" },
  { path: "research/why/phase-transition", file: "pages/research/why/phase-transition.tsx", id: "phase-transition" },
  { path: "research/why/rigidity-wall", file: "pages/research/why/rigidity-wall.tsx", id: "rigidity-wall" },
  { path: "research/why/sigma-cycles", file: "pages/research/why/sigma-cycles.tsx", id: "sigma-cycles" },
  { path: "research/why/forbidden-patterns", file: "pages/research/why/forbidden-patterns.tsx", id: "forbidden-patterns" },
  { path: "research/why/no-forced-moves", file: "pages/research/why/no-forced-moves.tsx", id: "no-forced-moves" },
  { path: "research/why/piece-theft", file: "pages/research/why/piece-theft.tsx", id: "piece-theft" },
  { path: "research/why/entropy-area-law", file: "pages/research/why/entropy-area-law.tsx", id: "entropy-area-law" },
  { path: "research/why/rare-color-geography", file: "pages/research/why/rare-color-geography.tsx", id: "rare-color-geography" },
  { path: "research/why/border-balance", file: "pages/research/why/border-balance.tsx", id: "border-balance" },
  { path: "research/build", file: "pages/research/build/Hub.tsx", id: "build" },
  { path: "research/build/run-it-yourself", file: "pages/research/build/run-it-yourself.tsx", id: "run-it-yourself" },
  { path: "research/build/dead-ends", file: "pages/research/build/dead-ends.tsx", id: "dead-ends" },
  { path: "research/build/solvers", file: "pages/research/build/solvers/Hub.tsx", id: "solvers" },
  { path: "research/lab", file: "pages/research/lab/Hub.tsx", id: "lab" },
  { path: "research/lab/experiments", file: "pages/research/lab/experiments/Hub.tsx", id: "experiments" },
  { path: "research/lab/experiments/log", file: "pages/research/lab/experiments/log.tsx", id: "experiments-log" },
  { path: "research/lab/findings", file: "pages/research/lab/findings/Hub.tsx", id: "findings" },
  { path: "research/lab/experiments/palimpsest", file: "pages/research/lab/experiments/palimpsest.tsx", id: "exp-palimpsest" },
  { path: "research/lab/experiments/keyring", file: "pages/research/lab/experiments/keyring.tsx", id: "exp-keyring" },
  { path: "research/lab/experiments/gauntlet", file: "pages/research/lab/experiments/gauntlet.tsx", id: "exp-gauntlet" },
  { path: "research/lab/experiments/prior", file: "pages/research/lab/experiments/prior.tsx", id: "exp-prior" },
  { path: "research/lab/experiments/staged", file: "pages/research/lab/experiments/staged.tsx", id: "exp-staged" },
  { path: "research/lab/experiments/bandsaw", file: "pages/research/lab/experiments/bandsaw.tsx", id: "exp-bandsaw" },
  { path: "research/lab/experiments/ladder", file: "pages/research/lab/experiments/ladder.tsx", id: "exp-ladder" },
  { path: "research/lab/experiments/replay", file: "pages/research/lab/experiments/replay.tsx", id: "exp-replay" },
  { path: "research/lab/experiments/cloister", file: "pages/research/lab/experiments/cloister.tsx", id: "exp-cloister" },
  { path: "research/lab/experiments/midden", file: "pages/research/lab/experiments/midden.tsx", id: "exp-midden" },
  { path: "research/lab/experiments/mosaic", file: "pages/research/lab/experiments/mosaic.tsx", id: "exp-mosaic" },
  { path: "research/lab/experiments/lodestone", file: "pages/research/lab/experiments/lodestone.tsx", id: "exp-lodestone" },
  { path: "research/reference", file: "pages/ReferenceTable.tsx", id: "reference" },
  { path: "research/papers", file: "pages/Papers.tsx", id: "papers" },
  { path: "research/records", file: "pages/Records.tsx", id: "records" },
  { path: "viewer", file: "pages/Viewer.tsx", id: "viewer" },
];

function tree(prefix: "" | "fr") {
  const idTag = prefix === "fr" ? "fr-" : "en-";
  const seg = prefix === "fr" ? "fr" : "";
  return PAGES.map((p) => {
    const fullPath = [seg, p.path].filter(Boolean).join("/");
    const opts = { id: idTag + p.id };
    return p.path === "" && prefix === ""
      ? index(p.file, opts)
      : route(fullPath || seg, p.file, opts);
  });
}

export default [layout("layout.tsx", [...tree(""), ...tree("fr")])] satisfies RouteConfig;
