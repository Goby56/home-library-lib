#!/usr/bin/env bash
set -e

# Get host machine IP
HOST_IP=$(ip route get 1.1.1.1 | awk '{print $7; exit}')

BACKEND_URL="http://${HOST_IP}:8080" npm run dev -- --host

