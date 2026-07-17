import { defineConfig, devices } from "@playwright/test";

// One smoke test that drives the app's critical cross-cutting flow (routing +
// i18n + lazy chunks + search). Playwright starts the dev server itself and
// tears it down after. Kept deliberately small — this is a smoke test, not a
// full e2e matrix.
export default defineConfig({
  testDir: "./e2e",
  // Serial against the single dev server: these smoke tests share one Vite dev
  // server, and running them in parallel makes the search-dialog open race under
  // request contention. A 4-test smoke suite is fast enough serially.
  fullyParallel: false,
  workers: 1,
  forbidOnly: !!process.env['CI'],
  retries: process.env['CI'] ? 1 : 0,
  reporter: process.env['CI'] ? "line" : "list",
  use: {
    baseURL: "http://localhost:5173",
    trace: "on-first-retry",
  },
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"] } }],
  webServer: {
    command: "pnpm dev --port 5173 --strictPort",
    url: "http://localhost:5173",
    reuseExistingServer: !process.env['CI'],
    timeout: 120_000,
  },
});
