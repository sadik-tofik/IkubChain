#!/bin/bash

set -e

echo "Building IkubChain..."

# Build parachain
echo "Building parachain..."
cd parachain
cargo build --release
cd ..

# Build frontend
echo "Building frontend..."
cd frontend
npm install
npm run build
cd ..

echo "Build complete!"

