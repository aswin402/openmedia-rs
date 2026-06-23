# ==========================================
# Builder Stage
# ==========================================
FROM rust:1.82-slim-bookworm AS builder

WORKDIR /usr/src/openmedia

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy root configurations
COPY Cargo.toml Cargo.lock ./
COPY crates/openmedia-core/Cargo.toml ./crates/openmedia-core/
COPY crates/openmedia-image/Cargo.toml ./crates/openmedia-image/
COPY crates/openmedia-video/Cargo.toml ./crates/openmedia-video/
COPY crates/openmedia-svg/Cargo.toml ./crates/openmedia-svg/
COPY crates/openmedia-animate/Cargo.toml ./crates/openmedia-animate/
COPY crates/openmedia-process/Cargo.toml ./crates/openmedia-process/
COPY crates/openmedia-improve/Cargo.toml ./crates/openmedia-improve/
COPY crates/openmedia-mcp/Cargo.toml ./crates/openmedia-mcp/

# Dummy build to cache dependencies
RUN mkdir -p crates/openmedia-core/src && echo "fn main() {}" > crates/openmedia-core/src/lib.rs && \
    mkdir -p crates/openmedia-image/src && echo "fn main() {}" > crates/openmedia-image/src/lib.rs && \
    mkdir -p crates/openmedia-video/src && echo "fn main() {}" > crates/openmedia-video/src/lib.rs && \
    mkdir -p crates/openmedia-svg/src && echo "fn main() {}" > crates/openmedia-svg/src/lib.rs && \
    mkdir -p crates/openmedia-animate/src && echo "fn main() {}" > crates/openmedia-animate/src/lib.rs && \
    mkdir -p crates/openmedia-process/src && echo "fn main() {}" > crates/openmedia-process/src/lib.rs && \
    mkdir -p crates/openmedia-improve/src && echo "fn main() {}" > crates/openmedia-improve/src/lib.rs && \
    mkdir -p crates/openmedia-mcp/src && echo "fn main() {}" > crates/openmedia-mcp/src/lib.rs && \
    echo "fn main() {}" > crates/openmedia-mcp/src/main.rs

RUN cargo build --release --bin openmedia-mcp

# Copy real source code
COPY crates/ ./crates/

# Force rebuild with real source code
RUN touch crates/openmedia-mcp/src/main.rs
RUN cargo build --release --bin openmedia-mcp

# ==========================================
# Runner Stage
# ==========================================
FROM debian:bookworm-slim AS runner

# Install runtime dependencies: FFmpeg and Chromium (headless browser rendering)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ffmpeg \
    chromium \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables for runtime dependencies
ENV OPENMEDIA_CHROME_PATH=/usr/bin/chromium
ENV OPENMEDIA_MODEL_DIR=/root/.openmedia/models
ENV OPENMEDIA_OUTPUT_DIR=/root/.openmedia/output
ENV OPENMEDIA_HISTORY_DB=/root/.openmedia/history.db

WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /usr/src/openmedia/target/release/openmedia-mcp /usr/local/bin/openmedia-mcp

# Create storage directories
RUN mkdir -p /root/.openmedia/models /root/.openmedia/output

# Run the MCP server over stdio transport
ENTRYPOINT ["openmedia-mcp"]
