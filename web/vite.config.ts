import path from "node:path";
import { writeFileSync } from "node:fs";
import { defineConfig, type Plugin } from "vite";
import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import mdx from "@mdx-js/rollup";
import remarkFrontmatter from "remark-frontmatter";
import remarkMdxFrontmatter from "remark-mdx-frontmatter";
import remarkGfm from "remark-gfm";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import rehypeSlug from "rehype-slug";
import rehypeShiki from "@shikijs/rehype";
import { allRoutePaths } from "./sitemap.config";
import { researchContent } from "./plugins/research-content";

// Emit sitemap.xml into the build output from the same page list React Router
// prerenders, so crawlers (and the AI bots welcomed in public/robots.txt) get
// an accurate map with zero hand-maintenance. Origin + base mirror site.ts.
function sitemap(): Plugin {
  return {
    name: "emit-sitemap",
    apply: "build",
    enforce: "post",
    writeBundle(options) {
      // The build runs twice (client + a temporary server bundle that gets
      // removed under ssr:false). Only emit into the client output.
      const outDir = options.dir ?? "";
      if (!outDir.includes("client")) return;
      const origin = (process.env["VITE_SITE_ORIGIN"] || "https://eternity2.dev").replace(/\/$/, "");
      const base = (process.env["BASE_PATH"] ?? "").replace(/\/$/, "");
      // Emit the trailing-slash form the host actually serves at 200. GitHub
      // Pages 301-redirects an extensionless path to its slash variant, so a
      // sitemap <loc> without the slash lists a redirect (which suppresses
      // indexing). Keep the root and any path with a file extension as-is.
      const canonical = (p: string) =>
        p === "/" || /\.[a-z0-9]+$/i.test(p) || p.endsWith("/") ? p : p + "/";
      const urls = allRoutePaths(base)
        .map((p) => `  <url><loc>${origin}${canonical(p)}</loc></url>`)
        .join("\n");
      const xml =
        `<?xml version="1.0" encoding="UTF-8"?>\n` +
        `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`;
      writeFileSync(path.join(outDir, "sitemap.xml"), xml);
    },
  };
}

// React Router framework mode (ssr:false) owns the React plugin, the entry
// points and the build.
//
// `base` is the absolute prefix for built assets. Default "/" serves the site
// at a domain root (eternity2.dev). To deploy under a path prefix (e.g. behind
// a reverse proxy at `host/eternity2/`), build with BASE_PATH=/eternity2; it
// must match the router `basename` in react-router.config.ts. The trailing
// slash is required by Vite. NB: `base` only rewrites the asset URLs baked into
// the HTML, not the output directory — Vite still writes assets to
// build/client/assets/. scripts/relocate-under-base.mjs (run after the build)
// moves the whole output under the prefix dir so the files match those URLs,
// letting the proxy pass the prefix straight through (no StripPrefix).
const BASE_PATH = (process.env["BASE_PATH"] ?? "").replace(/\/$/, "");

// Build-time engine backend switch. `VITE_ENGINE` picks which implementation of
// the engine surface the site is built against; the rest of the app imports
// the virtual module "virtual:engine-backend" (re-exported by src/engine), and
// the alias below resolves it to the chosen backend. All four backends expose
// the identical surface and are validated against the same Rust golden data.
//   rust (default) -> Rust/WASM   ts -> pure TypeScript
//   c -> C/WASM                   cpp -> C++/WASM
const ENGINE_BACKENDS: Record<string, string> = {
  rust: "./src/engine/backends/rust.ts",
  ts: "./src/engine-ts/index.ts",
  c: "./src/engine-c/glue.ts",
  cpp: "./src/engine-cpp/glue.ts",
};
const ENGINE = process.env["VITE_ENGINE"] ?? "rust";
const ENGINE_BACKEND = ENGINE_BACKENDS[ENGINE];
if (!ENGINE_BACKEND) {
  throw new Error(
    `VITE_ENGINE="${ENGINE}" is not one of: ${Object.keys(ENGINE_BACKENDS).join(", ")}`,
  );
}

export default defineConfig({
  base: BASE_PATH ? BASE_PATH + "/" : "/",
  // Mirror BASE_PATH into a VITE_-prefixed var so render code (src/site.ts) can
  // read it via import.meta.env for canonical/hreflang/og URLs.
  define: {
    "import.meta.env.VITE_BASE_PATH": JSON.stringify(BASE_PATH),
  },
  plugins: [
    // MDX must transform .mdx files before React Router / React see them.
    // Research wiki pages (web/content/research) are authored in MDX; the
    // pipeline gives GFM tables, KaTeX math ($…$ / $$…$$), heading anchor ids
    // and build-time Shiki syntax highlighting (dual theme via CSS variables).
    {
      enforce: "pre" as const,
      ...mdx({
        remarkPlugins: [remarkFrontmatter, remarkMdxFrontmatter, remarkGfm, remarkMath],
        rehypePlugins: [
          rehypeSlug,
          rehypeKatex,
          [
            rehypeShiki,
            { themes: { light: "github-light", dark: "github-dark" }, defaultColor: "light" },
          ],
        ],
      }),
    },
    researchContent(),
    reactRouter(),
    tailwindcss(),
    sitemap(),
  ],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "virtual:engine-backend": path.resolve(__dirname, ENGINE_BACKEND),
    },
  },
});
