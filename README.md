# IkubChain - Decentralized Investment Club Platform on Polkadot

IkubChain is a comprehensive decentralized investment club platform built on Polkadot, transforming traditional community-based investment models into transparent, secure, and globally accessible Web3 investment DAOs.

## Project Overview

IkubChain leverages the full technological stack of the Polkadot ecosystem to address challenges faced by communal investment structures (Ikub, Tandas, Susu, investment clubs) by implementing a sophisticated multi-chain architecture that eliminates trust dependencies, geographical limitations, and operational inefficiencies.

## Architecture

### Components

1. **Custom Substrate Parachain** - Optimized for investment club operations
2. **Core Pallets**:

   - `pallet-ikub-governance` - Sophisticated voting mechanisms
   - `pallet-ikub-treasury` - Multi-signature treasury operations
   - `pallet-ikub-crosschain` - Cross-chain investment execution
   - `pallet-ikub-members` - Member management and reputation
   - `pallet-ikub-disputes` - Decentralized dispute resolution
   - `pallet-ikub-analytics` - On-chain analytics and reporting

3. **Frontend Application** - Next.js 14 with TypeScript
4. **Smart Contracts** - ink! contracts for investment logic
5. **Cross-Chain Integration** - XCM and bridge architecture

## Project Structure

```
IkubChain/
├── parachain/              # Substrate parachain implementation
│   ├── pallets/           # Custom pallets
│   ├── runtime/           # Runtime configuration
│   └── node/              # Node implementation
├── frontend/              # Next.js frontend application
├── contracts/             # ink! smart contracts
├── scripts/               # Development and deployment scripts
└── docs/                  # Documentation

```

## Development Setup

### Prerequisites

- Rust (latest stable version) - Install from [rustup.rs](https://rustup.rs/)
- Node.js 18+ and npm/yarn
- Substrate prerequisites:
  ```bash
  curl https://getsubstrate.io -sSf | bash -s -- --fast
  ```
- Polkadot.js Extension for browser wallet connection

### Building the Parachain

```bash
cd parachain
cargo build --release
```

This will build all pallets and the runtime. The build may take 15-30 minutes on first run.

### Running the Node

```bash
cd parachain
./target/release/ikubchain-node --dev
```

The node will start on `ws://127.0.0.1:9944`

### Running the Frontend

```bash
cd frontend
npm install
npm run dev
```

The frontend will be available at `http://localhost:3000`

### Seed Data Setup

Run the development setup script:

```bash
./scripts/dev.sh
```

Or manually create seed data using Polkadot.js Apps:

1. Open https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944
2. Connect to local node
3. Use the Developer > Extrinsics tab to create clubs, proposals, etc.

## MVP Features

### ✅ Completed Features

1. **Club Management**

   - Create investment clubs
   - Join/leave clubs
   - Member reputation tracking

2. **Governance**

   - Create proposals (Investment, Operational, Emergency, Constitutional)
   - Multiple voting mechanisms (Simple Majority, Quadratic, Conviction, Delegated)
   - Quadratic voting implementation (cost = votes²)
   - Proposal finalization and tallying

3. **Treasury**

   - Deposit funds to club treasury
   - Contribution cycles with configurable periods
   - Minimum contribution requirements
   - Returns distribution (proportional to contributions)
   - Multi-signature withdrawal requests

4. **Cross-Chain (XCM v3)**

   - Send funds to other parachains
   - Execute remote investment calls
   - Operation tracking and status

5. **Disputes**

   - Open disputes between members
   - Submit evidence
   - Vote on disputes
   - Resolve disputes based on votes

6. **Analytics**

   - Track events (new members, clubs, contributions, proposals, cross-chain calls, disputes)
   - Club-specific and global metrics
   - Performance metrics

7. **Frontend**
   - Wallet connection (Polkadot.js Extension)
   - Club listing and creation
   - Club detail pages with tabs (Overview, Proposals, Treasury)
   - Proposal creation and voting
   - Treasury contribution interface
   - Governance dashboard

## API Reference

### Pallets

#### ikub-members

- `create_club(name, description)` - Create a new investment club
- `join_club(club_id)` - Join an existing club
- `leave_club(club_id)` - Leave a club

#### ikub-governance

- `create_proposal(club_id, proposal_type, voting_mechanism, title, description, voting_duration, approval_threshold)` - Create a proposal
- `vote(club_id, proposal_id, choice)` - Cast a vote (supports quadratic voting)
- `finalize_proposal(club_id, proposal_id)` - Finalize and tally votes

#### ikub-treasury

- `deposit(club_id, amount)` - Deposit funds to treasury
- `open_contribution_cycle(club_id, contribution_period, minimum_contribution)` - Open a new cycle
- `contribute(club_id, amount)` - Contribute to active cycle
- `close_cycle(club_id)` - Close the active cycle
- `distribute_returns(club_id, cycle_id, returns)` - Set returns for distribution
- `claim_returns(club_id, cycle_id)` - Claim proportional returns

#### ikub-crosschain

- `send_funds_to_parachain(club_id, dest_para_id, amount, beneficiary)` - Send funds via XCM
- `execute_remote_investment(club_id, dest_para_id, call_data)` - Execute remote call

#### ikub-disputes

- `open_dispute(club_id, subject, description)` - Open a dispute
- `submit_evidence(club_id, dispute_id, evidence_description)` - Submit evidence
- `vote_on_dispute(club_id, dispute_id, choice)` - Vote on dispute
- `resolve_dispute(club_id, dispute_id)` - Resolve dispute

#### ikub-analytics

- `update_metrics(club_id, metrics)` - Update performance metrics
- `track_event(club_id, event_type)` - Track analytics event (internal)

## Testing

### Run Pallet Tests

```bash
cd parachain/pallets/ikub-members
cargo test

# Repeat for other pallets
```

### Frontend Testing

```bash
cd frontend
npm run test
```

## Troubleshooting

### Build Issues

If you encounter build errors:

1. Ensure you have the latest Rust stable: `rustup update stable`
2. Clean and rebuild: `cargo clean && cargo build --release`
3. Check Substrate prerequisites are installed

### Node Connection Issues

If the frontend can't connect:

1. Verify node is running: `curl http://localhost:9944`
2. Check WebSocket connection: `wscat -c ws://127.0.0.1:9944`
3. Ensure firewall allows connections on port 9944

### Wallet Connection Issues

1. Install Polkadot.js Extension from Chrome/Firefox store
2. Create or import an account
3. Ensure extension is enabled for the site

## Next Steps

- Add comprehensive test coverage
- Implement XCM v3 fully (currently simulated)
- Add mobile app support
- Integrate with DeFi protocols
- Add fiat on-ramp
- Implement advanced analytics dashboard

## Development Roadmap

### Phase 1: Core MVP (Weeks 1-6)

- Basic governance mechanisms
- Single-chain treasury management
- Essential member management
- Functional web application

### Phase 2: Enhanced Features (Months 2-4)

- Cross-chain investment execution
- Advanced voting mechanisms
- Mobile application
- DeFi protocol integrations

### Phase 3: Ecosystem Growth (Months 5-9)

- White-label solutions
- Advanced investment strategies
- Fiat on-ramp integrations
- Regulatory compliance features

## License

[To be determined]

## Contributing

[Contributing guidelines to be added]
