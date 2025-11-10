#!/bin/bash

set -e

echo "Starting IkubChain development environment..."

# Start frontend in background
echo "Starting frontend..."
cd frontend
npm install
npm run dev &
FRONTEND_PID=$!
cd ..

# Note: Parachain node would be started separately
echo "Frontend started on http://localhost:3000"
echo "To start the parachain node, run:"
echo "  cd parachain && cargo run --release -- --dev --ws-port 9944"

# Wait for user interrupt
trap "kill $FRONTEND_PID" EXIT
wait $FRONTEND_PID

