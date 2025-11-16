#!/bin/bash

# IkubChain Development Seed Data Script
# This script sets up the development environment with seed data

set -e

echo "🚀 IkubChain Development Setup"
echo "================================"

# Check if node is running
if ! curl -s http://localhost:9944 > /dev/null; then
    echo "❌ Error: Node is not running on port 9944"
    echo "Please start the node first:"
    echo "  cd parachain && cargo build --release && ./target/release/ikubchain-node --dev"
    exit 1
fi

echo "✅ Node is running"

# Install dependencies if needed
if [ ! -d "frontend/node_modules" ]; then
    echo "📦 Installing frontend dependencies..."
    cd frontend && npm install && cd ..
fi

echo "📝 Creating seed data..."

# Use polkadot-js-api or similar tool to create seed data
# For now, we'll provide instructions

cat << EOF

To create seed data, use the Polkadot.js Apps or API:

1. Create a test club:
   - Go to: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944
   - Navigate to: Developer > Extrinsics
   - Select: ikubMembers > createClub
   - Enter:
     * name: "Test Investment Club"
     * description: "A test club for development"

2. Join the club:
   - Select: ikubMembers > joinClub
   - Enter clubId: 0

3. Create a proposal:
   - Select: ikubGovernance > createProposal
   - Enter proposal details

4. Contribute to treasury:
   - Select: ikubTreasury > openContributionCycle
   - Then: ikubTreasury > contribute

Alternatively, use the frontend at http://localhost:3000

EOF

echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Start the frontend: cd frontend && npm run dev"
echo "  2. Open http://localhost:3000"
echo "  3. Connect your wallet and start using IkubChain!"
