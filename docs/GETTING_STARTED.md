# Getting Started with IkubChain

## Overview

IkubChain is a decentralized investment club platform built on Polkadot. This guide will help you get started with development.

## Prerequisites

Before you begin, ensure you have:

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Node.js**: Version 18 or higher
- **Substrate Prerequisites**: See [Substrate Documentation](https://docs.substrate.io/install/)

## Project Structure

```
IkubChain/
├── parachain/          # Substrate parachain
│   ├── pallets/        # Custom pallets
│   ├── runtime/        # Runtime configuration
│   └── node/           # Node implementation
├── frontend/           # Next.js frontend
├── contracts/          # ink! smart contracts
├── scripts/            # Development scripts
└── docs/               # Documentation
```

## Quick Start

### 1. Build the Parachain

```bash
cd parachain
cargo build --release
```

### 2. Start the Parachain Node

```bash
# After node implementation is complete
cargo run --release -- --dev --ws-port 9944
```

### 3. Start the Frontend

```bash
cd frontend
npm install
npm run dev
```

The frontend will be available at http://localhost:3000

## Development Workflow

1. Make changes to pallets or frontend
2. Rebuild the parachain if pallets changed
3. Restart the node
4. Test in the frontend

## Key Concepts

### Investment Clubs

Groups of members who pool resources and make collective investment decisions.

### Proposals

Investment opportunities or operational changes that require member voting.

### Treasury

Multi-signature accounts that hold club funds with time-locked withdrawals.

### Governance

Voting mechanisms including simple majority, quadratic, and conviction voting.

## Next Steps

- Read the [Architecture Documentation](./ARCHITECTURE.md)
- Check the [Implementation Status](./IMPLEMENTATION_STATUS.md)
- Review the [Contributing Guide](../CONTRIBUTING.md)
