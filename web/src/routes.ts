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
  { path: "start", file: "pages/Start.tsx", id: "start" },
  { path: "status", file: "pages/Status.tsx", id: "status" },
  { path: "puzzle", file: "pages/Puzzle.tsx", id: "puzzle" },
  { path: "is-it-a-scam", file: "pages/Scam.tsx", id: "scam" },
  { path: "playground", file: "pages/playground/Hub.tsx", id: "playground" },
  { path: "playground/watch", file: "pages/playground/Watch.tsx", id: "watch" },
  { path: "playground/solve", file: "pages/playground/Solve.tsx", id: "solve" },
  { path: "playground/paths", file: "pages/playground/Paths.tsx", id: "paths" },
  { path: "playground/print", file: "pages/playground/Print.tsx", id: "print" },
  { path: "algorithms", file: "pages/Algorithms.tsx", id: "algorithms" },
  { path: "viewer", file: "pages/Viewer.tsx", id: "viewer" },
  { path: "convert", file: "pages/Convert.tsx", id: "convert" },
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

// Research wiki pages (web/content/research/**.mdx) resolve through one
// catch-all per language. Explicit PAGES entries outrank the splat, so a
// legacy TSX page keeps winning until its entry is removed — migration to
// MDX is page-by-page with no dead URLs. Adding an MDX file needs NO entry
// here: the route resolves from the content manifest, and the prerender list
// picks the path up via sitemap.config.ts → content.config.ts.
const researchDocs = [
  route("research/*", "pages/research/doc.tsx", { id: "en-research-doc" }),
  route("fr/research/*", "pages/research/doc.tsx", { id: "fr-research-doc" }),
];

export default [
  layout("layout.tsx", [...tree(""), ...tree("fr"), ...researchDocs]),
] satisfies RouteConfig;
