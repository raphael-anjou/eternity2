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
  { path: "research/why/phase-transition", file: "pages/research/why/phase-transition.tsx", id: "phase-transition" },
  { path: "research/why/rigidity-wall", file: "pages/research/why/rigidity-wall.tsx", id: "rigidity-wall" },
  { path: "research/why/sigma-cycles", file: "pages/research/why/sigma-cycles.tsx", id: "sigma-cycles" },
  { path: "research/why/forbidden-patterns", file: "pages/research/why/forbidden-patterns.tsx", id: "forbidden-patterns" },
  { path: "research/why/no-forced-moves", file: "pages/research/why/no-forced-moves.tsx", id: "no-forced-moves" },
  { path: "research/why/entropy-area-law", file: "pages/research/why/entropy-area-law.tsx", id: "entropy-area-law" },
  { path: "research/why/rare-color-geography", file: "pages/research/why/rare-color-geography.tsx", id: "rare-color-geography" },
  { path: "research/build", file: "pages/research/build/Hub.tsx", id: "build" },
  { path: "research/build/run-it-yourself", file: "pages/research/build/run-it-yourself.tsx", id: "run-it-yourself" },
  { path: "research/build/dead-ends", file: "pages/research/build/dead-ends.tsx", id: "dead-ends" },
  { path: "research/lab", file: "pages/research/lab/Hub.tsx", id: "lab" },
  { path: "research/lab/inventions", file: "pages/research/lab/inventions/Hub.tsx", id: "inventions" },
  { path: "research/lab/inventions/palimpsest", file: "pages/research/lab/inventions/palimpsest.tsx", id: "inv-palimpsest" },
  { path: "research/lab/inventions/keyring", file: "pages/research/lab/inventions/keyring.tsx", id: "inv-keyring" },
  { path: "research/lab/inventions/gauntlet", file: "pages/research/lab/inventions/gauntlet.tsx", id: "inv-gauntlet" },
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
