FROM python:3.11-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
EXPOSE 8549
HEALTHCHECK --interval=20s --timeout=5s --start-period=10s \
  CMD curl -f http://localhost:8549/health || exit 1
CMD ["uvicorn", "src.main:app", "--host", "0.0.0.0", "--port", "8549", \
     "--workers", "1", "--timeout-keep-alive", "10"]
