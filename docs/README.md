# Oxidized Wallet Documentation

## 📁 Documentation Structure

### `/architecture/`
Technical design and implementation details:

- **[TECHNICAL_ARCHITECTURE.md](architecture/TECHNICAL_ARCHITECTURE.md)** - Technology choices (Vue 3 + Rust), architecture decisions, and rationale
- **[DATABASE_SCHEMA.md](architecture/DATABASE_SCHEMA.md)** - Complete SQLite schema with amount_minor and double-entry constraints
- **[PROJECT_STRUCTURE.md](architecture/PROJECT_STRUCTURE.md)** - Codebase organization with Vue 3 frontend structure
- **[DEVELOPMENT_DECISIONS.md](architecture/DEVELOPMENT_DECISIONS.md)** - Final technical decisions and implementation guidelines

### `/planning/`
Project planning and business rules:

- **[FEATURE_ROADMAP.md](planning/FEATURE_ROADMAP.md)** - Development phases from MVP to advanced features
- **[DOUBLE_ENTRY_RULES.md](planning/DOUBLE_ENTRY_RULES.md)** - Accounting principles and implementation rules

## 🎯 Quick Reference

### For Development Setup
1. Read [TECHNICAL_ARCHITECTURE.md](architecture/TECHNICAL_ARCHITECTURE.md) for technology overview
2. Check [PROJECT_STRUCTURE.md](architecture/PROJECT_STRUCTURE.md) for codebase organization
3. Review [CLAUDE.md](../CLAUDE.md) for development commands and context

### For Database Work
1. Study [DATABASE_SCHEMA.md](architecture/DATABASE_SCHEMA.md) for complete schema
2. Reference [DOUBLE_ENTRY_RULES.md](planning/DOUBLE_ENTRY_RULES.md) for business logic

### For Feature Planning
1. Check [FEATURE_ROADMAP.md](planning/FEATURE_ROADMAP.md) for current phase and priorities
2. Review MVP requirements before implementing new features

## 📋 Development Phases

| Phase | Status | Key Features |
|-------|--------|--------------|
| **MVP** | 📋 Planned | Account hierarchy, basic transactions, double-entry validation |
| **Phase 2** | 📋 Planned | Enhanced reporting, CSV export, transaction management |
| **Phase 3** | 📋 Planned | Bank imports, budget tracking, advanced reporting |
| **Phase 4** | 📋 Planned | Multi-currency, investment tracking, performance optimization |

## 🔗 Related Files
- **[CLAUDE.md](../CLAUDE.md)** - Complete development context and commands
- **[README.md](../README.md)** - Project overview and setup instructions

This documentation provides the complete foundation for building Oxidized Wallet with proper double-entry bookkeeping and French banking support.