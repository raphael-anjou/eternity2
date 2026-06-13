import type { Config } from "@react-router/dev/config";

// Static site: no server. `ssr: false` makes React Router pre-render every
// listed path to a real .html file at build time (good for crawlers and
// link-preview bots) and also emit an SPA fallback for anything else.
//
// Language lives in the URL as a path segment (English at the root, French
// under /fr) rather than as a `basename`, so each language has its own
// crawlable URL.
//
// Deploying under a path prefix (e.g. behind a Traefik StripPrefix at
// `host/eternity2/`): set BASE_PATH=/eternity2 at build time. It becomes the
// router `basename` here and the Vite asset `base` in vite.config.ts, so the
// absolute asset/route paths baked into the prerendered HTML carry the prefix.
// Default "" → served at the domain root. Keep this in sync with vite.config.
const BASE_PATH = (process.env.BASE_PATH ?? "").replace(/\/$/, "");

const PAGES = [
  "",
  "puzzle",
  "playground",
  "playground/watch",
  "playground/solve",
  "playground/paths",
  "playground/print",
  "algorithms",
  "research",
  "viewer",
];

const prerender = PAGES.flatMap((p) => {
  const en = "/" + p;
  const fr = "/fr/" + p;
  // Normalize the two roots ("/" and "/fr").
  return [en.replace(/\/$/, "") || "/", fr.replace(/\/$/, "")];
});

export default {
  // Keep the existing source layout: root.tsx and routes.ts live in src/
  // alongside pages/, components/, engine/ instead of a separate app/ dir.
  appDirectory: "src",
  ...(BASE_PATH ? { basename: BASE_PATH } : {}),
  ssr: false,
  prerender,
} satisfies Config;
