# DevilChain Core Node — Ultra-slim multi-stage build
# Final image: ~35MB
FROM rust:1.78-alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
# Cache deps layer
RUN mkdir src && echo 'fn main(){}' > src/main.rs && \
    cargo build --release 2>/dev/null || true && rm -rf src
COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl 2>/dev/null || \
    cargo build --release

FROM alpine:3.19
RUN apk add --no-cache curl ca-certificates
WORKDIR /app
COPY --from=builder /build/target/release/devilchain-node ./
RUN mkdir -p /data/chain
EXPOSE 8545 8546
HEALTHCHECK --interval=15s --timeout=5s --start-period=20s \
  CMD curl -f http://localhost:8545/api/status || exit 1
CMD ["./devilchain-node", "--mode", "testnet", "--data", "/data/chain"]
