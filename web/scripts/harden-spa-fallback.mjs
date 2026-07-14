// Post-build: mark the SPA fallback shell as noindex.
//
// React Router (ssr:false) emits build/client/__spa-fallback.html — a bare app
// shell with no title and no real content, served for any path that wasn't
// prerendered. On the nginx/Docker deploy its `try_files` hands unmatched URLs
// to this shell with a 200 status; a crawler that lands on it would otherwise
// index an empty, title-less page (a classic soft-404). GitHub Pages instead
// serves our real public/404.html (already noindex), so this is belt-and-
// suspenders — but it costs nothing and closes the gap on every deploy path.
//
// We inject a single <meta name="robots" content="noindex"> into <head>. The
// shell still hydrates and client-routes normally; only its crawl directive
// changes. Idempotent: skips if the tag is already present.

import path from "node:path";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const file = path.resolve(here, "..", "build", "client", "__spa-fallback.html");

if (!existsSync(file)) {
  // Not fatal: a future React Router version may rename or drop the fallback.
  console.log("harden-spa-fallback: __spa-fallback.html not found — skipping");
  process.exit(0);
}

let html = readFileSync(file, "utf8");
if (html.includes('name="robots"')) {
  console.log("harden-spa-fallback: robots meta already present — no change");
  process.exit(0);
}

const tag = '<meta name="robots" content="noindex"/>';
// Insert right after <head…> so it lands within the first bytes, before the
// title/meta React Router wrote. Fall back to prepending if <head> is absent.
if (/<head[^>]*>/i.test(html)) {
  html = html.replace(/<head[^>]*>/i, (m) => m + tag);
} else {
  html = tag + html;
}
writeFileSync(file, html);
console.log("harden-spa-fallback: injected noindex into __spa-fallback.html");
