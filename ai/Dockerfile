# DevilGuard AI — slim Python image
# Final image: ~90MB
FROM python:3.11-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
EXPOSE 8547
HEALTHCHECK --interval=20s --timeout=5s --start-period=15s \
  CMD curl -f http://localhost:8547/health || exit 1
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8547", \
     "--workers", "1", "--limit-concurrency", "20", "--timeout-keep-alive", "10"]
