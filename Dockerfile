# syntax=docker/dockerfile:1.7

FROM rust:1.88-bookworm AS backend-builder
WORKDIR /workspace

COPY Cargo.toml ./
COPY apps/backend ./apps/backend
# Keep Docker backend build independent from desktop toolchain packages:
# include only minimal desktop crate files required for Cargo workspace resolution.
COPY apps/desktop/src-tauri/Cargo.toml ./apps/desktop/src-tauri/Cargo.toml
COPY apps/desktop/src-tauri/build.rs ./apps/desktop/src-tauri/build.rs
COPY apps/desktop/src-tauri/src ./apps/desktop/src-tauri/src
COPY crates ./crates
COPY migrations ./migrations

RUN cargo build --release --package market-backend

FROM node:22-bookworm-slim AS web-builder
WORKDIR /workspace/apps/web

COPY apps/web/package.json apps/web/package-lock.json ./
RUN npm ci

COPY apps/web ./
RUN npm run build

FROM node:22-bookworm-slim AS web-runtime
WORKDIR /app

ENV NODE_ENV=production \
    PORT=3000

COPY --from=web-builder /workspace/apps/web/build ./build

EXPOSE 3000
CMD ["node", "build"]

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --create-home --home-dir /var/lib/market-osource --shell /usr/sbin/nologin market
WORKDIR /app

COPY --from=backend-builder /workspace/target/release/market-backend /usr/local/bin/market-backend

ENV RUNTIME_MODE=vps \
    BIND_ADDR=0.0.0.0:8080 \
    STORAGE_PATH=/var/lib/market-osource/storage

USER market
EXPOSE 8080
VOLUME ["/var/lib/market-osource/storage"]

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl --fail --silent http://127.0.0.1:8080/health >/dev/null || exit 1

CMD ["market-backend"]
