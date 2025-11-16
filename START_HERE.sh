#!/bin/bash
echo "🚀 IkubChain Build Script"
echo "========================"
echo ""
echo "This script will guide you through building IkubChain"
echo ""
echo "Step 1: Checking prerequisites..."

# Check Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust is installed: $(rustc --version)"
else
    echo "❌ Rust is NOT installed"
    echo "   Install with: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check Node
if command -v node &> /dev/null; then
    echo "✅ Node.js is installed: $(node --version)"
else
    echo "❌ Node.js is NOT installed"
    exit 1
fi

# Check npm
if command -v npm &> /dev/null; then
    echo "✅ npm is installed: $(npm --version)"
else
    echo "❌ npm is NOT installed"
    exit 1
fi

echo ""
echo "Step 2: Building parachain..."
cd parachain
if cargo build --release; then
    echo "✅ Parachain built successfully!"
else
    echo "❌ Build failed. Check errors above."
    exit 1
fi

echo ""
echo "Step 3: Installing frontend dependencies..."
cd ../frontend
if npm install; then
    echo "✅ Frontend dependencies installed!"
else
    echo "❌ npm install failed"
    exit 1
fi

echo ""
echo "✅ Build complete!"
echo ""
echo "Next steps:"
echo "1. Start node: cd parachain && ./target/release/ikubchain-node --dev"
echo "2. Start frontend: cd frontend && npm run dev"
echo "3. Open http://localhost:3000"
