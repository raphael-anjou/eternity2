# Multi-stage build: Rust engine -> WASM, then the React app, then a tiny
# nginx image serving the fully static site. No runtime services needed.
#
#   docker build -t eternity2-community .
#   docker run --rm -p 8080:80 eternity2-community
#
# The build is self-contained: nothing is required on the host besides Docker.

# ---- stage 1: engine (Rust -> WebAssembly) --------------------------------
FROM rust:1.95-slim AS engine
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown \
    && curl -sSfL https://rustwasm.github.io/wasm-pack/installer/init.sh | sh
WORKDIR /src/engine
COPY engine/ .
# Run the test suite first: it cross-validates the piece set, rotation
# conventions and scoring against real community boards.
RUN cargo test --release
RUN wasm-pack build --target web --out-dir /out/pkg --release

# ---- stage 2: web app (Vite/React) ----------------------------------------
FROM node:26-slim AS web
# Node 25+ no longer bundles Corepack, so install it before enabling it. Corepack
# then reads the "packageManager" field in package.json and provisions that exact
# pnpm version — keeping the version pin in one place.
RUN npm install -g corepack@latest && corepack enable
WORKDIR /src/web
# Optional: serve under a path prefix and/or a non-default public origin.
#   docker build --build-arg BASE_PATH=/eternity2 \
#                --build-arg VITE_SITE_ORIGIN=https://www.terra-numerica.fr .
# Default (empty) builds for a domain root at https://eternity2.dev.
ARG BASE_PATH=""
ARG VITE_SITE_ORIGIN=""
ENV BASE_PATH=$BASE_PATH
ENV VITE_SITE_ORIGIN=$VITE_SITE_ORIGIN
COPY web/package.json web/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY web/ .
COPY --from=engine /out/pkg src/engine/pkg
RUN pnpm build

# ---- stage 3: static server ------------------------------------------------
FROM nginx:1.27-alpine
# BASE_PATH must reach this stage too: the served files live under it and the
# nginx config (asset-cache location, SPA fallback) needs the prefix. Re-declare
# the ARG (ARGs don't cross stages) and default to empty for a root deploy.
ARG BASE_PATH=""
ENV BASE_PATH=$BASE_PATH
# Ship the config as a TEMPLATE. The nginx image entrypoint runs envsubst over
# everything in /etc/nginx/templates/*.template at startup, substituting
# ${BASE_PATH}, and writes the result to /etc/nginx/conf.d/. So the same image
# recipe yields a correct root config (BASE_PATH empty) or a prefixed one.
# Restrict envsubst to BASE_PATH so nginx's own $uri variables survive.
ENV NGINX_ENVSUBST_FILTER="BASE_PATH"
COPY deploy/nginx.conf.template /etc/nginx/templates/default.conf.template
COPY --from=web /src/web/build/client /usr/share/nginx/html
EXPOSE 80
# Probe the site's real entry point. For a prefixed build the app lives under
# ${BASE_PATH}/, so a bare / would 404 — hit ${BASE_PATH}/ instead. Use
# 127.0.0.1, not localhost: in Alpine, localhost resolves to ::1 (IPv6) first,
# but nginx only `listen 80;` (IPv4), so a localhost probe gets "connection
# refused", the container is marked unhealthy, and health-aware proxies
# (Traefik) then refuse to route to it. Shell form so $BASE_PATH expands.
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
  CMD wget -qO /dev/null "http://127.0.0.1${BASE_PATH}/" || exit 1
