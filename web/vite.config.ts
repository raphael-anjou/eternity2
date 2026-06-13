import path from "node:path";
import { defineConfig } from "vite";
import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";

// React Router framework mode (ssr:false) owns the React plugin, the entry
// points and the build.
//
// `base` is the absolute prefix for built assets. Default "/" serves the site
// at a domain root (eternity2.dev). To deploy under a path prefix (e.g. behind
// a Traefik StripPrefix at `host/eternity2/`), build with BASE_PATH=/eternity2;
// it must match the router `basename` in react-router.config.ts. The trailing
// slash is required by Vite.
const BASE_PATH = (process.env.BASE_PATH ?? "").replace(/\/$/, "");

export default defineConfig({
  base: BASE_PATH ? BASE_PATH + "/" : "/",
  // Mirror BASE_PATH into a VITE_-prefixed var so render code (src/site.ts) can
  // read it via import.meta.env for canonical/hreflang/og URLs.
  define: {
    "import.meta.env.VITE_BASE_PATH": JSON.stringify(BASE_PATH),
  },
  plugins: [reactRouter(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
});
