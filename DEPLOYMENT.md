# Deployment

The site is **fully static**: a folder of HTML/JS/CSS plus a 152 KB WebAssembly engine.
No server-side code, no database, no API, no environment variables. Deploy it like any
static site; a Docker image is also provided for container-based infrastructures.

## Option A: static hosting (recommended)

### GitHub Pages (automatic)

The repository ships a workflow (`.github/workflows/deploy.yml`) that builds the engine,
runs its test suite, builds the web app and publishes to GitHub Pages on every push to
the default branch. One-time setup after pushing the repo to GitHub:

1. Repository **Settings → Pages → Source: "GitHub Actions"**.
2. Push to `main`/`master`. The site is served at the domain root (a `CNAME` file
   pins it to `eternity2.dev`; remove `web/public/CNAME` to use the default
   `https://<user>.github.io/<repo>/`, but see the root-path note below).

The app is pre-rendered: every route × language is emitted as a real `.html`
file (`/algorithms/index.html`, `/fr/algorithms/index.html`, …) with the correct
`<html lang>` and `hreflang` tags, so crawlers and link-preview bots get real,
language-specific content. The workflow also copies the React Router SPA shell to
`404.html` so unmatched deep links still boot the app.

**Served at the domain root.** Because assets and routes use absolute paths
(`base: "/"`), the site must live at the root of a domain — it no longer works
unchanged under an arbitrary subpath. A project page at
`<user>.github.io/<repo>/` would need `base` and the route `basename` set to the
subpath and a rebuild (note: react-router's `ssr:false` prerender + `basename` is
currently buggy — prefer a root domain).

### Any other static host (GitLab Pages, CDN, plain nginx)

```bash
cd engine && wasm-pack build --target web --out-dir ../web/src/engine/pkg --release
cd ../web && pnpm install && pnpm build
# upload web/build/client/ anywhere that serves static files
```

Host requirements: serve `.wasm` with the `application/wasm` MIME type (all
mainstream hosts/CDNs do), and serve the site at the domain root. For unmatched
paths, fall back to `__spa-fallback.html` (the bundled `nginx.conf` does this; on
a CDN, point the 404/SPA fallback there).

## Option B: Docker

For infrastructures that deploy containers, a self-contained multi-stage `Dockerfile`
is provided (Rust/WASM build → web build → nginx). Nothing is needed on the build host
except Docker.

```bash
# build (runs the engine test suite as part of the build)
docker build -t eternity2-community .

# run
docker run --rm -p 8080:80 eternity2-community
# → http://localhost:8080
```

Or with compose:

```bash
docker compose up -d --build
```

To push to a private registry:

```bash
docker tag eternity2-community registry.example.org/eternity2/site:1.0.0
docker push registry.example.org/eternity2/site:1.0.0
```

### Behind a reverse proxy

The image expects to be served at the domain root (absolute asset/route paths).
A `StripPrefix`-style path prefix (`https://host/eternity2/`) no longer works
without a rebuild — set `base` in `vite.config.ts` and the route `basename` to
the prefix and rebuild. Expose only the container's port 80 to the proxy network.

### Operational profile

- One stateless nginx container; scale or restart freely, no volumes.
- Healthcheck built into the image (HTTP GET /).
- All computation (the solver!) runs in the visitor's browser, so the container's
  resource needs are those of a static file server: ~10 MB RAM.
- Logs: standard nginx access/error logs on stdout/stderr.

## Updating a deployment

1. Pull the new code.
2. Static: push to the default branch (Pages workflow redeploys). Docker:
   `docker build` again; the engine tests run inside the build, so a broken engine
   fails the build rather than shipping.
3. Cached assets are content-hashed, so browsers pick up new versions immediately;
   `index.html` is served with `no-cache`.
