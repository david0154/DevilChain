FROM rust:1.78-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY . .
RUN cargo build --release 2>/dev/null || echo 'build skipped'

FROM python:3.11-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY requirements.txt* ./
RUN pip install --no-cache-dir fastapi uvicorn pydantic 2>/dev/null || true
COPY . .
EXPOSE 8548
HEALTHCHECK --interval=20s --timeout=5s \
  CMD curl -f http://localhost:8548/stats || exit 1
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8548", "--workers", "1"]
