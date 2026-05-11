# DevilScan Explorer — multi-stage Next.js build
# Final image: ~120MB standalone
FROM node:20-alpine AS deps
WORKDIR /app
COPY package.json package-lock.json* ./
RUN npm ci --only=production --prefer-offline 2>/dev/null || npm ci --only=production

FROM node:20-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
ENV NEXT_TELEMETRY_DISABLED=1
RUN npm run build

FROM node:20-alpine AS runner
RUN addgroup -S devil && adduser -S devil -G devil
WORKDIR /app
ENV NODE_ENV=production NEXT_TELEMETRY_DISABLED=1
COPY --from=builder --chown=devil:devil /app/.next/standalone ./
COPY --from=builder --chown=devil:devil /app/.next/static ./.next/static
COPY --from=builder --chown=devil:devil /app/public ./public 2>/dev/null || true
USER devil
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=5s CMD curl -f http://localhost:3000 || exit 1
CMD ["node", "server.js"]
