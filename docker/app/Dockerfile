FROM rust:1.85.0-slim AS builder

WORKDIR /workspace/ems-backend

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN apt-get update && apt-get install -y \
    ca-certificates \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:bookworm-slim

COPY docker/app/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh
COPY --from=builder /workspace/ems-backend/target/release/ems-backend /usr/local/bin/ems-backend

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
