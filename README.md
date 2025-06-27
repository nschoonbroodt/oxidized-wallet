# Oxidized Wallet

A local-first personal finance tracking application with double-entry bookkeeping, built with Rust and Tauri.

## Features

- **Double-entry bookkeeping**: Ensures financial data integrity
- **Hierarchical accounts**: Organize accounts by bank and type
- **Local-first**: All data stored locally in SQLite
- **French banking focus**: Designed for EUR and French banking patterns
- **Modern UI**: Built with Tauri and Vue 3 for native performance

## Project Status

🚧 **In Development** - Currently in planning phase

See [docs/planning/FEATURE_ROADMAP.md](docs/planning/FEATURE_ROADMAP.md) for development phases.

## Documentation

- **[Architecture](docs/architecture/)** - Technical design and implementation
- **[Planning](docs/planning/)** - Feature roadmap and business rules
- **[Development Guide](CLAUDE.md)** - Setup and development commands

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/oxidized-wallet.git
cd oxidized-wallet

# Setup development environment (coming soon)
./scripts/setup-dev.sh

# Run tests (coming soon)
cargo test

# Start development server (coming soon)
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

## Contact

[Your contact information]