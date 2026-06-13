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
FROM node:22-slim AS web
RUN corepack enable
WORKDIR /src/web
COPY web/package.json web/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY web/ .
COPY --from=engine /out/pkg src/engine/pkg
RUN pnpm build

# ---- stage 3: static server ------------------------------------------------
FROM nginx:1.27-alpine
COPY deploy/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=web /src/web/build/client /usr/share/nginx/html
EXPOSE 80
HEALTHCHECK --interval=30s --timeout=3s \
  CMD wget -qO /dev/null http://localhost/ || exit 1
