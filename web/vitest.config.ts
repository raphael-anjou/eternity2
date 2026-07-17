import { defineConfig } from "vitest/config";
import path from "node:path";

// Standalone Vitest config (does NOT reuse vite.config.ts, whose plugins pull in
// the WASM engine and the MDX content pipeline — none of which the unit tests
// need). Just the `@` → src alias and a node environment for the pure logic.
export default defineConfig({
  resolve: {
    alias: { "@": path.resolve(__dirname, "./src") },
  },
  test: {
    environment: "node",
    include: ["src/**/*.test.ts", "src/**/*.test.tsx"],
    // Playwright specs live under e2e/ and run with their own runner, not vitest.
    exclude: ["e2e/**", "node_modules/**"],
  },
});
