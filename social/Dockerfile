FROM node:20-alpine AS deps
WORKDIR /app
COPY package.json ./
RUN npm install --prefer-offline

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
USER devil
EXPOSE 3001
CMD ["node", "server.js"]
