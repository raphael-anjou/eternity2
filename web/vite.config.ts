import path from "node:path";
import { defineConfig } from "vite";
import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";

// React Router framework mode (ssr:false) owns the React plugin, the entry
// points and the build. base "/" because the site is served at the root of a
// custom domain (eternity2.dev); nested paths like /fr/algorithms need an
// absolute base, unlike the old relative "./" hash-routed build.
export default defineConfig({
  base: "/",
  plugins: [reactRouter(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
});
