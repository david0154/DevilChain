# Contributing to DevilChain Network

Welcome! Here's how to get started.

## Prerequisites
- Rust 1.77+
- Node.js 20+
- Flutter 3.19+
- Docker & Docker Compose

## Setup

```bash
git clone https://github.com/david0154/DevilChain.git
cd DevilChain

# Blockchain core
cd core && cargo build && cargo run

# Explorer
cd ../explorer && npm install && npm run dev

# Smart contracts
cd ../contracts && npm install && npx hardhat compile

# Docker (all services)
cd ../docker && docker-compose up -d
```

## Workflow
1. Fork the repo
2. Create branch: `git checkout -b feature/your-feature`
3. Write tests
4. Commit with clear messages
5. Open PR against `develop`

## Code Style
- Rust: `cargo fmt` + `cargo clippy`
- TypeScript: ESLint + Prettier
- Dart: `dart format`
- Solidity: Solhint

## License
Contributions are licensed under MIT.
