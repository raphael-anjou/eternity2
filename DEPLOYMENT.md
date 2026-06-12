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
2. Push to `main`/`master`. The site appears at `https://<user>.github.io/<repo>/`.

No configuration is needed for the subpath: the app uses relative asset paths and hash
routing, so it works at any URL.

### Any other static host (GitLab Pages, CDN, plain nginx)

```bash
cd engine && wasm-pack build --target web --out-dir ../web/src/engine/pkg --release
cd ../web && pnpm install && pnpm build
# upload web/dist/ anywhere that serves static files
```

The only host requirement: serve `.wasm` files with the `application/wasm` MIME type
(all mainstream hosts and CDNs do). Routing is hash-based, so no 404/rewrite rules are
needed.

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

### Behind a reverse proxy with a path prefix

The same image works at the domain root or behind any path prefix (e.g.
`https://host/eternity2/`) **without a rebuild and without configuration**, thanks to
relative asset paths and hash routing. With Traefik, use a standard `StripPrefix`
middleware; ready-made labels are in `docker-compose.yaml` (commented). There is no
`BASE_PATH` to set and no port to expose besides the container's port 80 to the proxy
network.

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
