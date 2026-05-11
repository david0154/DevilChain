# DevilChain Node — Dockerfile
# Developed by Nexuzy Lab (nexuzy.tech) | Powered by Devil One (devilone.in)

FROM rust:1.78-slim AS builder
WORKDIR /app
COPY core/ ./core/
COPY Cargo.toml Cargo.lock ./
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release --manifest-path core/Cargo.toml

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/core/target/release/devilchain /app/devilchain
EXPOSE 8545 8546 30303
VOLUME ["/data/chain"]
CMD ["/app/devilchain"]
