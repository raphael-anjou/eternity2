import type { Config } from "@react-router/dev/config";

// Static site: no server. `ssr: false` makes React Router pre-render every
// listed path to a real .html file at build time (good for crawlers and
// link-preview bots) and also emit an SPA fallback for anything else.
//
// Language lives in the URL as a path segment (English at the root, French
// under /fr) rather than a `basename` — basename + prerender + ssr:false is
// broken in react-router since 7.2.0 (remix-run/react-router#13615), so we
// model the prefix as ordinary nested routes instead.
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
  ssr: false,
  prerender,
} satisfies Config;
