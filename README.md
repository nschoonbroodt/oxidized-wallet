# Oxidized Wallet

A local-first personal finance tracking application with double-entry bookkeeping, built with Rust and Tauri.

## Features

- **Double-entry bookkeeping**: Ensures financial data integrity
- **Hierarchical accounts**: Organize accounts by bank and type
- **Local-first**: All data stored locally in SQLite
- **French banking focus**: Designed for EUR and French banking patterns
- **Modern UI**: Built with Tauri and Vue 3 for native performance

## Project Status

✅ MVP Complete

## Documentation

- **[Architecture](docs/architecture/)** - Technical design and implementation
- **[Planning](docs/planning/)** - Feature roadmap and business rules
- **[Development Guide](CLAUDE.md)** - Setup and development commands

## Quick Start

```bash
# Clone the repository
git clone https://github.com/nschoonbroodt/oxidized-wallet.git
cd oxidized-wallet

# Run tests
cargo test

# Setup development environment
cd wallet-tauri && npm install

# Start development app
cd wallet-tauri && cargo tauri dev
```

## Technology Stack

- **Backend**: Rust with sqlx
- **Frontend**: Tauri + Vue 3 + TypeScript
- **Database**: SQLite
- **Architecture**: Split codebase (wallet-core + wallet-tauri)

## License

MIT License - See LICENSE file for details

## Contributing

This is currently a personal learning project. Contributions may be accepted in the future.

