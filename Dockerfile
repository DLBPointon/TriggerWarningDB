# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM rust:1-bookworm AS builder

WORKDIR /app

# Install SQLite3 build dependency and diesel_cli build deps
RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Install diesel_cli (sqlite only, to keep compile time down)
RUN cargo install diesel_cli --no-default-features --features sqlite

# --- Dependency caching layer ---
# Copy manifests only, build a dummy binary so dependencies are compiled and
# cached in their own layer before we copy our actual source code.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# --- Build the real application ---
COPY src ./src
# Touch main.rs so Cargo knows it changed and rebuilds our crate
RUN touch src/main.rs
RUN cargo build --release

# =============================================================================
# Stage 2: Runtime
# =============================================================================
FROM debian:bookworm-slim

WORKDIR /app

# Runtime dependencies: sqlite3 shared library + CA certs for HTTPS calls
RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled application binary and diesel CLI
COPY --from=builder /app/target/release/trigger_warning ./trigger_warning
COPY --from=builder /usr/local/cargo/bin/diesel          ./diesel

# Copy application assets
COPY templates   ./templates
COPY static      ./static
COPY images      ./images
COPY migrations  ./migrations
COPY Rocket.toml ./Rocket.toml

# Persistent volume for the SQLite database file
RUN mkdir -p /data
VOLUME ["/data"]

# Write a small entrypoint that runs migrations then starts the server
RUN printf '#!/bin/sh\nset -e\necho "Running database migrations..."\n./diesel migration run --database-url "$DATABASE_URL"\necho "Starting Trigger Warning..."\nexec ./trigger_warning\n' > entrypoint.sh \
    && chmod +x entrypoint.sh

# -------------------------------------------------------------------------
# Rocket configuration overrides
#
# ROCKET_ADDRESS overrides the 127.0.0.1 default from Rocket.toml so the
# server binds to all interfaces inside the container.
#
# ROCKET_DATABASES overrides the sqlite path to point at the /data volume
# so the database survives container restarts.
#
# Set JWT_SECRET to a strong random value at runtime, e.g.:
#   docker run -e JWT_SECRET=<your-secret> ...
# -------------------------------------------------------------------------
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV DATABASE_URL=/data/db.sqlite
ENV ROCKET_DATABASES='{sqlite_database={url="/data/db.sqlite",pool_size=2}}'

EXPOSE 8000

ENTRYPOINT ["./entrypoint.sh"]
