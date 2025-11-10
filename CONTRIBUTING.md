# Contributing to IkubChain

Thank you for your interest in contributing to IkubChain! This document provides guidelines and information for contributors.

## Development Setup

1. **Install Prerequisites**

   - Rust (latest stable)
   - Node.js 18+
   - Substrate prerequisites

2. **Clone and Build**

   ```bash
   git clone <repository-url>
   cd IkubChain
   ./scripts/build.sh
   ```

3. **Run Development Environment**
   ```bash
   ./scripts/dev.sh
   ```

## Code Style

- **Rust**: Follow standard Rust formatting (`cargo fmt`)
- **TypeScript**: Follow ESLint and Prettier configurations
- **Commits**: Use conventional commit messages

## Testing

- Write unit tests for all pallets
- Add integration tests for critical workflows
- Ensure frontend components are tested

## Pull Request Process

1. Create a feature branch
2. Make your changes
3. Add tests
4. Update documentation
5. Submit PR with clear description

## Areas Needing Contribution

- Runtime implementation completion
- Node implementation
- Pallet feature enhancements
- Frontend UI components
- Smart contract development
- Testing and documentation
