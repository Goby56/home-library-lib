#!/usr/bin/env bash
set -e

# Get host machine IP
HOST_IP=$(ip route get 1.1.1.1 | awk '{print $7; exit}')

ALLOWED_ORIGIN="http://${HOST_IP}:5173" cargo run --bin hll
