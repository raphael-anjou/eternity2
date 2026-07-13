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

**Downloadable build artifact.** Every workflow run also uploads the built site
as a plain `web-dist` artifact (the exact `web/build/client/` bytes that go to
Pages, retained 14 days). Grab it from the run's **Actions → Artifacts** section
to self-host the same build behind your own server/proxy without rebuilding —
unzip it and point nginx/Caddy/Traefik at the folder. (This is the *root* build;
for a path-prefix host, build with `BASE_PATH` yourself — see below.)

**Crawler / AI-agent files.** The build root carries three discovery files:
`robots.txt` and `llms.txt` ship verbatim from `web/public/`, while `sitemap.xml`
is generated at build time by a Vite plugin from the same page list React Router
prerenders (so it never drifts). `robots.txt` explicitly welcomes every major AI
crawler (GPTBot, ClaudeBot, PerplexityBot, Google-Extended, …) plus a blanket
allow; `llms.txt` is a curated [llmstxt.org](https://llmstxt.org) site map for
LLMs. The sitemap honors `VITE_SITE_ORIGIN`/`BASE_PATH`, so subpath/alternate-host
deploys get correct absolute URLs automatically. `robots.txt` hardcodes the
`eternity2.dev` sitemap URL — edit it if you deploy elsewhere.

**Path prefix / subpath deploys.** By default the site is served at a domain
root and asset/route paths are absolute. To serve it under a prefix instead
(e.g. behind a reverse proxy at `host/eternity2/`, or a GitHub project page at
`<user>.github.io/<repo>/`), set two build-time env vars — no code changes:

```bash
# served at https://www.terra-numerica.fr/eternity2/
BASE_PATH=/eternity2 \
VITE_SITE_ORIGIN=https://www.terra-numerica.fr \
pnpm build
# the whole site is now under build/client/eternity2/ — upload that, or upload
# build/client/ and let the server resolve /eternity2/… paths.
```

- `BASE_PATH` — the path prefix. Becomes the Vite asset `base` and the router
  `basename`, so every asset link, nav link and prerendered route in the output
  carries the prefix. The `pnpm build` post-step
  (`scripts/relocate-under-base.mjs`) then moves the entire build output under a
  matching `build/client/eternity2/` directory, so the files on disk sit exactly
  where those prefixed URLs point. This means the prefix is **passed straight
  through** to the files — a reverse proxy must **not** strip it (see the Traefik
  note below). Default `""` (root); the post-step is then a no-op.
- `VITE_SITE_ORIGIN` — the public origin used for `canonical`, `hreflang` and
  `og:url` tags. Default `https://eternity2.dev`. Set it so the SEO/social tags
  point at the real deployed URL.

### Any other static host (GitLab Pages, CDN, plain nginx)

```bash
cd engine && wasm-pack build --target web --out-dir ../web/src/engine/pkg --release
cd ../web && pnpm install && pnpm build
# upload web/build/client/ anywhere that serves static files
```

Host requirements: serve `.wasm` with the `application/wasm` MIME type (all
mainstream hosts/CDNs do), and serve the site at the domain root. For unmatched
paths, fall back to `__spa-fallback.html` (the bundled nginx template
`deploy/nginx.conf.template` does this; on a CDN, point the 404/SPA fallback
there). For a prefixed build, everything — including the SPA fallback — lives
under the prefix dir, so point the fallback at `<prefix>/__spa-fallback.html`.

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

**Prefixed image** (to serve under `https://example.org/eternity2/`): pass the
same two knobs as build args. `BASE_PATH` bakes the prefix into the site *and*
into the nginx config (asset caching, SPA fallback) and the container
healthcheck, so the image is self-consistent and needs no proxy rewriting:

```bash
docker build \
  --build-arg BASE_PATH=/eternity2 \
  --build-arg VITE_SITE_ORIGIN=https://example.org \
  -t eternity2-community:eternity2 .
docker run --rm -p 8080:80 eternity2-community:eternity2
# → http://localhost:8080/eternity2/   (bare / returns 404 by design)
```

To push to a private registry:

```bash
docker tag eternity2-community registry.example.org/eternity2/site:1.0.0
docker push registry.example.org/eternity2/site:1.0.0
```

### Behind a reverse proxy (Traefik)

Both setups below are verified end-to-end against a real Traefik v3 instance —
assets, the WASM engine, French routes, the sitemap and the SPA fallback all
resolve, and the app hydrates in the browser. The container ships a healthcheck
and Traefik only routes to healthy containers, so both cases "just work" once
the container reports healthy (a few seconds after start).

**At a host root** (`https://eternity2.example/`): nothing special — route the
host to the container, expose only its port 80 to the proxy network.

```yaml
# docker-compose labels
labels:
  - traefik.enable=true
  - traefik.http.routers.eternity2.rule=Host(`eternity2.example`)
  - traefik.http.routers.eternity2.entrypoints=websecure
  - traefik.http.routers.eternity2.tls.certresolver=letsencrypt
  - traefik.http.services.eternity2.loadbalancer.server.port=80
```

**Under a path prefix** (`https://example.org/eternity2/`): build the image with
`BASE_PATH=/eternity2` (build arg, see above) and **do NOT use `StripPrefix`**.
The prefixed build lays the whole site out under `/eternity2/` on disk and emits
`/eternity2/...` asset and route links, so it expects to *receive* the prefix —
stripping it would make every asset 404. Route the prefixed path straight
through:

```yaml
# docker-compose labels (no StripPrefix middleware)
build:
  context: .
  args:
    BASE_PATH: /eternity2
    VITE_SITE_ORIGIN: https://example.org
labels:
  - traefik.enable=true
  - traefik.http.routers.eternity2.rule=Host(`example.org`) && PathPrefix(`/eternity2`)
  - traefik.http.routers.eternity2.entrypoints=websecure
  - traefik.http.routers.eternity2.tls.certresolver=letsencrypt
  - traefik.http.services.eternity2.loadbalancer.server.port=80
```

The rule of thumb: the prefix must survive all the way to the files. Build the
prefix in (`BASE_PATH`), and let the proxy pass it through rather than strip it.
Also set `VITE_SITE_ORIGIN` so the SEO tags use the public hostname.

### Operational profile

- One stateless nginx container; scale or restart freely, no volumes.
- Healthcheck built into the image (HTTP GET of the site root — `/`, or
  `<prefix>/` for a prefixed build — over IPv4). Traefik/Swarm/Kubernetes will
  only route to the container once it reports healthy.
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
