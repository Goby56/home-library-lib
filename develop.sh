#!/usr/bin/env bash
set -e

# Get host machine IP
HOST_IP=$(ip route get 1.1.1.1 | awk '{print $7; exit}')

# Run backend in background with logs prefixed
cd backend
ALLOWED_ORIGIN="http://${HOST_IP}:5173" \
    cargo run --bin hll \
    2>&1 | sed "s/^/[BACKEND] /" &
BACKEND_PID=$!

# Ensure backend is killed when script exits
trap "echo 'Stopping backend...'; kill $BACKEND_PID" EXIT

# Wait until backend is ready
BACKEND_URL="http://${HOST_IP}:8080"
echo "Waiting for backend at $BACKEND_URL ..."
until curl -s "$BACKEND_URL" > /dev/null; do
    sleep 0.5
done
echo "Backend is ready!"

# Serve frontend with logs prefixed
cd ../frontend
BACKEND_URL="http://${HOST_IP}:8080" \
    npm run dev -- --host \ 
    2>&1 | sed 's/^/[FRONTEND] /'
