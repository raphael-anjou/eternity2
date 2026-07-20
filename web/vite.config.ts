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
import { routeEntries } from "./sitemap.config";
import { researchPageLastmod, researchHubLastmod } from "./content.config";
import { mainPageLastmod, pageUpdatedKey } from "./src/page-updated";
import { researchContent } from "./plugins/research-content";

// Tag added/removed lines inside ```diff code blocks so CSS can paint the
// GitHub-style full-line green/red backgrounds. Shiki colors the +/- glyphs but
// leaves each line's background flat. This runs as a Shiki *transformer* (not a
// separate rehype pass, whose ordering relative to Shiki is not guaranteed under
// @mdx-js/rollup): the `line` hook fires per line with the source text in hand,
// so we stamp data-diff only when the block's language is `diff`.
// Minimal local hast shapes — enough for what the `line` hook touches, without
// pulling in @types/hast / @shikijs/types as direct deps.
type HastText = { type: "text"; value: string };
type HastElement = {
  type: "element";
  properties: Record<string, unknown>;
  children: HastNode[];
};
type HastNode = HastText | HastElement | { type: string; children?: HastNode[] };
const textOf = (n: HastNode): string => {
  if (n.type === "text") return (n as HastText).value;
  return ("children" in n && n.children ? n.children.map(textOf).join("") : "");
};
const diffLinesTransformer = {
  name: "diff-line-bg",
  line(this: { options: { lang: string } }, node: HastElement) {
    // `this.options.lang` is the fenced-block language for this run.
    if (this.options.lang !== "diff") return;
    const first = textOf(node).trimStart()[0];
    // hast serializes the camelCase `dataDiff` back to the `data-diff` attribute.
    if (first === "+") node.properties["dataDiff"] = "add";
    else if (first === "-") node.properties["dataDiff"] = "remove";
  },
};

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
      // Research MDX pages carry an `updated:` frontmatter date; main TSX pages
      // carry a declared date in src/page-updated.ts. Surface either as
      // <lastmod> so crawlers get a real freshness signal. Both maps are keyed
      // by the basename-relative page path (e.g. "research/build/known-facts",
      // "status", "" for home), which we recover from each full route path by
      // stripping the base + leading slash. A /fr mirror shares its English
      // source's date (pageUpdatedKey drops the "fr/" prefix), so a translated
      // /fr/research/* page inherits its EN twin's `updated:` date. Hubs/topics/
      // people pages have no declared date and get no <lastmod> — a
      // partial-coverage sitemap is valid and better than a fabricated date.
      const lastmod = { ...researchPageLastmod(), ...researchHubLastmod(), ...mainPageLastmod() };
      const lastmodFor = (fullPath: string): string | undefined => {
        let rel = fullPath;
        if (base && rel.startsWith(base)) rel = rel.slice(base.length);
        rel = rel.replace(/^\//, "");
        return lastmod[rel] ?? lastmod[pageUpdatedKey(rel)];
      };
      // Every localized <url> for a page carries the same set of <xhtml:link>
      // alternates — one per language the page is available in, plus x-default
      // (→ English), per Google's reciprocal-hreflang requirement. A page
      // available in one language only (an untranslated research page) lists
      // just itself + x-default. Built from the same routeEntries() the app's
      // <head> hreflang uses, so the two channels never disagree.
      const abs = (p: string) => `${origin}${canonical(p)}`;
      const altLinks = (alternates: { lang: string; loc: string }[], xDefault: string) =>
        alternates
          .map((a) => `<xhtml:link rel="alternate" hreflang="${a.lang}" href="${abs(a.loc)}"/>`)
          .join("") +
        `<xhtml:link rel="alternate" hreflang="x-default" href="${abs(xDefault)}"/>`;
      const urlEntry = (loc: string, alts: string) => {
        const lm = lastmodFor(loc);
        const locTag = `<loc>${abs(loc)}</loc>`;
        const lmTag = lm ? `<lastmod>${lm}</lastmod>` : "";
        return `  <url>${locTag}${lmTag}${alts}</url>`;
      };
      const urls = routeEntries(base)
        .map(({ loc, alternates, xDefault }) => {
          // A single-language page (no translated twin) gets no alternates.
          const alts = alternates.length > 1 ? altLinks(alternates, xDefault) : "";
          return urlEntry(loc, alts);
        })
        .join("\n");
      const xml =
        `<?xml version="1.0" encoding="UTF-8"?>\n` +
        `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" ` +
        `xmlns:xhtml="http://www.w3.org/1999/xhtml">\n${urls}\n</urlset>\n`;
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
            {
              themes: { light: "github-light", dark: "github-dark" },
              defaultColor: "light",
              addLanguageClass: true,
              transformers: [diffLinesTransformer],
            },
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
