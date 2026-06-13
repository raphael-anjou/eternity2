# AGENTS.md — notes for AI agents working on this repo

The Eternity II community site: a Rust→WASM solver engine (`engine/`) plus a
static React Router 7 site (`web/`). Static-only, no server, bilingual (English
at the root, French under `/fr`). See `README.md` for the user-facing tour and
`DEPLOYMENT.md` for build/deploy details.

## Conventions & gotchas

- **Single source of truth for the page list.** Crawlable page paths live in
  `web/sitemap.config.ts` (`PAGE_PATHS` + `allRoutePaths()`). Both
  `web/react-router.config.ts` (the prerender list) and the `emit-sitemap` Vite
  plugin in `web/vite.config.ts` import from it. **Add new pages there**, not by
  re-declaring an array — an inline page list will silently drift from the
  sitemap. Note React Router applies `basename` itself, so the prerender list
  uses *unprefixed* paths (`allRoutePaths()`), while the sitemap applies
  `BASE_PATH` (`allRoutePaths(base)`) because it emits absolute URLs.

- **Crawler/SEO files live in two places.** `web/public/robots.txt` and
  `web/public/llms.txt` are static (copied verbatim to the build root).
  `sitemap.xml` is *generated* at build time — do not add a static one to
  `public/`. The sitemap reads `VITE_SITE_ORIGIN`/`BASE_PATH`; `robots.txt`
  hardcodes the `eternity2.dev` sitemap URL.

- **`llms.txt` is index-only by design.** We deliberately do **not** generate
  per-page `.md` counterparts. Page content is authored in interactive React/TSX
  (playground, live solvers), so HTML→Markdown would produce thin or meaningless
  stubs. The prerendered HTML is clean enough for agents to parse. Don't add a
  per-page `.md` build step without revisiting that trade-off.

- **`ssr: false` build runs twice.** Vite emits a client bundle and a temporary
  server bundle (removed afterward). Build plugins that write files must gate on
  the client output dir (`options.dir.includes("client")`) or they fail trying
  to write into the deleted server dir.

- **Verify after web changes:** `cd web && pnpm build && pnpm typecheck`.
