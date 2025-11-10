# IkubChain Architecture Documentation

## Overview

IkubChain is built on a custom Substrate parachain with specialized pallets for investment club operations, connected to a modern Next.js frontend application.

## System Architecture

### Parachain Layer

The parachain consists of six core pallets:

1. **pallet-ikub-governance**: Handles proposal creation, voting mechanisms, and decision-making
2. **pallet-ikub-treasury**: Manages multi-signature treasury operations and fund management
3. **pallet-ikub-crosschain**: Enables cross-chain investment execution via XCM
4. **pallet-ikub-members**: Manages member onboarding, profiles, and reputation
5. **pallet-ikub-disputes**: Provides decentralized dispute resolution
6. **pallet-ikub-analytics**: Tracks performance metrics and generates reports

### Frontend Layer

The frontend is built with Next.js 14 and provides:

- Dashboard for overview and analytics
- Club management interfaces
- Investment workflow tools
- Governance voting interfaces
- Treasury operation views
- Member management

### Data Flow

1. User interactions in frontend
2. Polkadot.js API connects to parachain node
3. Transactions submitted to blockchain
4. Events emitted and reflected in UI

## Development Workflow

1. Start parachain node: `cd parachain && cargo run --release -- --dev`
2. Start frontend: `cd frontend && npm run dev`
3. Access frontend at http://localhost:3000
4. Connect to parachain at ws://localhost:9944

## Security Considerations

- Multi-signature controls for treasury operations
- Time-locked withdrawals
- Governance-controlled parameter updates
- Comprehensive event logging for auditability
