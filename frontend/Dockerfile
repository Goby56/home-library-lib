# Build stage
FROM node:22-alpine AS builder

WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm ci

# Copy source files and build the app
COPY . .
RUN npm run build
RUN npm prune --production

# Production stage
FROM node:22-alpine

WORKDIR /app

# Copy build artifacts and production dependencies
COPY --from=builder /app/build /app/build
COPY --from=builder /app/node_modules /app/node_modules
COPY package.json /app/package.json

# Expose the port the app will run on
EXPOSE 3000

# Set environment to production
ENV NODE_ENV=production

# Start the application
CMD ["node", "build"]

