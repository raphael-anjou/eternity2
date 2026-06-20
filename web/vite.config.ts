import path from "node:path";
import { writeFileSync } from "node:fs";
import { defineConfig, type Plugin } from "vite";
import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { allRoutePaths } from "./sitemap.config";

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
      const urls = allRoutePaths(base)
        .map((p) => `  <url><loc>${origin}${p}</loc></url>`)
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
// a Traefik StripPrefix at `host/eternity2/`), build with BASE_PATH=/eternity2;
// it must match the router `basename` in react-router.config.ts. The trailing
// slash is required by Vite.
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
  plugins: [reactRouter(), tailwindcss(), sitemap()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "virtual:engine-backend": path.resolve(__dirname, ENGINE_BACKEND),
    },
  },
});
