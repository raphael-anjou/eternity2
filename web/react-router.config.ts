import type { Config } from "@react-router/dev/config";
import { allRoutePaths } from "./sitemap.config";

// Static site: no server. `ssr: false` makes React Router pre-render every
// listed path to a real .html file at build time (good for crawlers and
// link-preview bots) and also emit an SPA fallback for anything else.
//
// Language lives in the URL as a path segment (English at the root, French
// under /fr) rather than as a `basename`, so each language has its own
// crawlable URL.
//
// Deploying under a path prefix (e.g. behind a reverse proxy at
// `host/eternity2/`): set BASE_PATH=/eternity2 at build time. It becomes the
// router `basename` here and the Vite asset `base` in vite.config.ts, so the
// absolute asset/route paths baked into the prerendered HTML carry the prefix.
// The prerendered HTML also lands under build/client/eternity2/ (that's what
// `basename` does); scripts/relocate-under-base.mjs then moves the assets and
// public files under the same prefix so the whole site is self-consistent and
// the proxy passes the prefix straight through (NO StripPrefix — stripping it
// would 404 every asset). Default "" → served at the domain root. Keep this in
// sync with vite.config.
const BASE_PATH = (process.env["BASE_PATH"] ?? "").replace(/\/$/, "");

// Page list lives in sitemap.config.ts so the prerender list and the
// build-time sitemap.xml are generated from one source and cannot drift.
// React Router applies the basename itself, so prerender paths carry no prefix.
const prerender = allRoutePaths();

export default {
  // Keep the existing source layout: root.tsx and routes.ts live in src/
  // alongside pages/, components/, engine/ instead of a separate app/ dir.
  appDirectory: "src",
  ...(BASE_PATH ? { basename: BASE_PATH } : {}),
  ssr: false,
  prerender,
} satisfies Config;
